function constructUrl() {
  const url = new URL(location);
  url.pathname = "/api/calendar.svg";
  removeEmptyParams(url);
  return url;
}

// as of 2025 there really seems to be no better way, mapping doesn't work...
function removeEmptyParams(url) {
  const keysToDelete = [];
  for (const [key, value] of url.searchParams.entries()) {
    if (!value) keysToDelete.push(key);
  }
  keysToDelete.forEach((key) => url.searchParams.delete(key));
}

async function fetchData(url) {
  const response = await fetch(url);

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

function setupUrlSection(url) {
  const input = document.querySelector("#svg-url > input");
  input.onclick = (e) => e.target.select();
  input.value = url.href;
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

const url = constructUrl();
setupUrlSection(url);
fetchData(url).catch(console.error);
