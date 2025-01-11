/// =======================================
/// lib/utils/datetime
/// =======================================

/**
 * Calculates the number of days between 2 specified dates, excluding the current date
 *
 * @param {Date} startDate the earlier date that we will substract from the end date
 * @param {Date} endDate the last date in the range
 * @return {Number} number of days in between
 */
function getDayDifference(startDate, endDate) {
  const millisecondsPerDay = 1000 * 60 * 60 * 24;
  const date1 = Date.UTC(
    startDate.getFullYear(),
    startDate.getMonth(),
    startDate.getDate(),
  );
  const date2 = Date.UTC(
    endDate.getFullYear(),
    endDate.getMonth(),
    endDate.getDate(),
  );

  return Math.floor((date2 - date1) / millisecondsPerDay);
}

/**
 * Converts a Date object to a date-only string in the ISO format `yyyy-mm-dd`
 *
 * @param {Date} date A Date object
 * @returns {string} A string in the format `yyyy-mm-dd`
 */
const toISODateFormat = (date) => {
  const day = padWithZeros(date.getDate());
  const month = padWithZeros(date.getMonth() + 1);
  const year = date.getFullYear();
  return `${year}-${month}-${day}`;
};

/**
 * Pads given items with zeros to reach a length of 2 characters.
 *
 * @param   {...any} args Items to be padded.
 * @returns {Array<String>} Padded items.
 */
const padWithZeros = (...args) => args.map((arg) => `${arg}`.padStart(2, "0"));

/**
 * Returns i18n weekday names array.
 */
const getWeekdayNames = () => [
  "Sunday",
  "Monday",
  "Tuesday",
  "Wednesday",
  "Thursday",
  "Friday",
  "Saturday",
];

/**
 * Given a date object returns the day of the week in English
 * @param   {Date} date
 * @returns {string}
 */
const getDayName = (date) => getWeekdayNames()[date.getDay()];

/// =======================================

const firstDayOfWeekChoices = Object.freeze({
  sunday: 0,
  monday: 1,
  saturday: 6,
});

const CONTRIB_LEGENDS = [
  { title: "No contributions", min: 0 },
  { title: "1-9 contributions", min: 1 },
  { title: "10-19 contributions", min: 10 },
  { title: "20-29 contributions", min: 20 },
  { title: "30+ contributions", min: 30 },
];

function getSystemDate(systemUtcOffsetSeconds) {
  const date = new Date();
  const localUtcOffsetMinutes = 0 - date.getTimezoneOffset();
  const systemUtcOffsetMinutes = systemUtcOffsetSeconds / 60;
  date.setMinutes(
    date.getMinutes() - localUtcOffsetMinutes + systemUtcOffsetMinutes,
  );
  return date;
}

function formatTooltipText({ date, count }) {
  const dateDayName = getDayName(date);
  const dateText = date.toString();

  let contribText = "No contributions";
  if (count > 0) {
    contribText = count + " contribution" + (count !== 1 ? "s" : "");
  }
  return `${contribText}<br /><span class="gl-text-gray-300 dark:gl-text-gray-700">${dateDayName} ${dateText}</span>`;
}

// Return the contribution level from the number of contributions
const getLevelFromContributions = (count) => {
  if (count <= 0) {
    return 0;
  }

  const nextLevel = CONTRIB_LEGENDS.findIndex(({ min }) => count < min);

  // If there is no higher level, we are at the end
  return nextLevel >= 0 ? nextLevel - 1 : CONTRIB_LEGENDS.length - 1;
};

class ActivityCalendar {
  constructor({
    container,
    timestamps,
    onClick: onClickDay = () => {},
    utcOffset = 0,
    firstDayOfWeek = firstDayOfWeekChoices.sunday,
    monthsAgo = 12,
  }) {
    this.currentSelectedDate = "";
    this.daySpace = 1;
    this.daySize = 14;
    this.daySizeWithSpace = this.daySize + this.daySpace * 2;
    this.monthNames = [
      "Jan",
      "Feb",
      "Mar",
      "Apr",
      "May",
      "Jun",
      "Jul",
      "Aug",
      "Sep",
      "Oct",
      "Nov",
      "Dec",
    ];
    this.months = [];
    this.firstDayOfWeek = firstDayOfWeek;
    this.container = container;
    this.onClickDay = onClickDay;

    // Loop through the timestamps to create a group of objects
    // The group of objects will be grouped based on the day of the week they are
    this.timestampsTmp = [];
    let group = 0;

    const today = getSystemDate(utcOffset);
    today.setHours(0, 0, 0, 0, 0);

    const timeAgo = new Date(today);
    timeAgo.setMonth(today.getMonth() - monthsAgo);

    const days = getDayDifference(timeAgo, today);

    for (let i = 0; i <= days; i += 1) {
      const date = new Date(timeAgo);
      date.setDate(date.getDate() + i);

      const day = date.getDay();
      const count = timestamps[toISODateFormat(date)] || 0;

      // Create a new group array if this is the first day of the week
      // or if is first object
      if ((day === this.firstDayOfWeek && i !== 0) || i === 0) {
        this.timestampsTmp.push([]);
        group += 1;
      }

      // Push to the inner array the values that will be used to render map
      const innerArray = this.timestampsTmp[group - 1];
      innerArray.push({ count, date, day });
    }

    // Init the svg element
    this.svg = this.renderSvg(container, group);
    this.renderDays();
    this.renderMonths();
    this.renderDayTitles();
  }

  // Add extra padding for the last month label if it is also the last column
  getExtraWidthPadding(group) {
    let extraWidthPadding = 0;
    const lastColMonth = this.timestampsTmp[group - 1][0].date.getMonth();
    const secondLastColMonth = this.timestampsTmp[group - 2][0].date.getMonth();

    if (lastColMonth !== secondLastColMonth) {
      extraWidthPadding = 6;
    }

    return extraWidthPadding;
  }

  renderSvg(container, group) {
    const width =
      (group + 1) * this.daySizeWithSpace + this.getExtraWidthPadding(group);
    return d3
      .select(container)
      .append("svg")
      .attr("width", width)
      .attr("height", 140)
      .attr("class", "contrib-calendar")
      .attr("data-testid", "contrib-calendar");
  }

  dayYPos(day) {
    return this.daySizeWithSpace * ((day + 7 - this.firstDayOfWeek) % 7);
  }

  renderDays() {
    this.svg
      .selectAll("g")
      .data(this.timestampsTmp)
      .enter()
      .append("g")
      .attr("transform", (group, i) => {
        group.forEach((stamp, a) => {
          if (a === 0 && stamp.day === this.firstDayOfWeek) {
            const month = stamp.date.getMonth();
            const x = this.daySizeWithSpace * i + 1 + this.daySizeWithSpace;
            const lastMonth = this.months[this.months.length - 1];
            if (
              lastMonth == null ||
              (month !== lastMonth.month &&
                x - this.daySizeWithSpace !== lastMonth.x)
            ) {
              this.months.push({ month, x });
            }
          }
        });
        return `translate(${this.daySizeWithSpace * i + 1 + this.daySizeWithSpace}, 18)`;
      })
      .attr("data-testid", "user-contrib-cell-group")
      .selectAll("rect")
      .data((stamp) => stamp)
      .enter()
      .append("rect")
      .attr("x", "0")
      .attr("y", (stamp) => this.dayYPos(stamp.day))
      .attr("rx", "2")
      .attr("ry", "2")
      .attr("width", this.daySize)
      .attr("height", this.daySize)
      .attr("data-level", (stamp) => getLevelFromContributions(stamp.count))
      .attr("title", (stamp) => formatTooltipText(stamp))
      .attr("class", "user-contrib-cell has-tooltip")
      .attr("data-testid", "user-contrib-cell")
      .attr("data-html", true)
      .attr("data-container", "body")
      .on("click", (element, stamp) => this.clickDay(element, stamp));
  }

  renderDayTitles() {
    const days = [
      {
        text: "M", // todo: internationalisation
        y: 29 + this.dayYPos(1),
      },
      {
        text: "W", // todo: internationalisation
        y: 29 + this.dayYPos(3),
      },
      {
        text: "F", // todo: internationalisation
        y: 29 + this.dayYPos(5),
      },
    ];

    if (this.firstDayOfWeek === firstDayOfWeekChoices.monday) {
      days.push({
        text: "S", // todo: internationalisation
        y: 29 + this.dayYPos(7),
      });
    } else if (this.firstDayOfWeek === firstDayOfWeekChoices.saturday) {
      days.push({
        text: "S", // todo: internationalisation
        y: 29 + this.dayYPos(6),
      });
    }

    this.svg
      .append("g")
      .selectAll("text")
      .data(days)
      .enter()
      .append("text")
      .attr("text-anchor", "middle")
      .attr("x", 8)
      .attr("y", (day) => day.y)
      .text((day) => day.text)
      .attr("class", "user-contrib-text");
  }

  renderMonths() {
    this.svg
      .append("g")
      .attr("direction", "ltr")
      .selectAll("text")
      .data(this.months)
      .enter()
      .append("text")
      .attr("x", (date) => date.x)
      .attr("y", 10)
      .attr("class", "user-contrib-text")
      .text((date) => this.monthNames[date.month]);
  }

  clickDay(element, stamp) {
    if (this.currentSelectedDate !== stamp.date) {
      this.currentSelectedDate = stamp.date;
      this.onClickDay(toISODateFormat(this.currentSelectedDate));

      // Remove is-active class from all other cells
      this.svg
        .selectAll(".user-contrib-cell.is-active")
        .classed("is-active", false);

      // Add is-active class to the clicked cell
      element.currentTarget.classList.add("is-active");
    } else {
      this.currentSelectedDate = "";

      // Remove is-active class from all other cells
      this.svg
        .selectAll(".user-contrib-cell.is-active")
        .classed("is-active", false);
    }
  }
}

// -------------------------------------------

function renderActivityCalendar(data) {
  // const utcOffset = $calendarWrap.data("utcOffset");
  const calendarHint = "Issues, merge requests, pushes, and comments.";

  new ActivityCalendar({
    container: ".js-contrib-calendar",
    timestamps: data,
    onClick: (date) =>
      console.log(`${data[date] || 0} contributions on ${date}`),
  });

  // Scroll to end
  const calendarContainer = document.querySelector(".js-contrib-calendar");
  calendarContainer.scrollLeft = calendarContainer.scrollWidth;
}

async function fetchData() {
  const response = await fetch(
    "http://localhost:3000/api/calendar?gitlab=thomas-zahner&github=thomas-zahner",
  );

  if (!response.ok) {
    throw new Error(`Response status: ${response.status}`);
  }

  const activity = await response.json();
  renderActivityCalendar(activity);
}

fetchData().catch(console.error);
