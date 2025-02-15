async function fetchData({ gitlab, github, cellSize, fontSize }) {
  const params = new URLSearchParams();

  if (gitlab) params.append("gitlab", gitlab);
  if (github) params.append("github", github);
  if (cellSize) params.append("cell_size", cellSize);
  if (fontSize) params.append("font_size", fontSize);

  const response = await fetch(
    `http://localhost:3000/api/calendar.svg?${params}`,
  );

  if (!response.ok) {
    throw new Error(`Response status: ${response.status}`);
  }

  const svg = await response.text();
  const calendarContainer = document.querySelector(".js-contrib-calendar");
  calendarContainer.innerHTML = svg;
  calendarContainer.scrollLeft = calendarContainer.scrollWidth;
}

function showPopup(event, textContent) {
  const popup = document.querySelector("#popup");
  Object.assign(popup.style, {
    left: `${event.clientX + window.scrollX}px`,
    top: `${event.clientY + window.scrollY}px`,
    display: "block",
  });

  popup.textContent = textContent;
}

function hidePopup() {
  const popup = document.querySelector("#popup");
  popup.style.display = "none";
}

function whenUserContribCell(event, then) {
  const target = event.target;
  const list = target.classList;

  if (list.contains("user-contrib-cell") && list.contains("has-tooltip")) {
    then();
  }
}

addEventListener("mouseover", (event) => {
  whenUserContribCell(event, () => {
    const target = event.target;
    const date = target.getAttribute("data-date");
    let textContent = target.getAttribute("data-hover-info");

    if (date) {
      textContent += " on " + date;
    }

    showPopup(event, textContent);
  });
});

addEventListener("mouseout", (event) => whenUserContribCell(event, hidePopup));

const params = new URL(location).searchParams;
fetchData({
  gitlab: params.get("gitlab"),
  github: params.get("github"),
  cellSize: params.get("cell_size"),
  fontSize: params.get("font_size"),
}).catch(console.error);
