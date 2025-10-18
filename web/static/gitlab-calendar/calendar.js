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
    const errorMessage = document.querySelector("#error-message");
    errorMessage.style.display = "initial";
    errorMessage.textContent = await response.text();
    throw new Error(`Response status: ${response.status}`);
  }

  const svg = await response.text();
  const calendar = document.querySelector(".calendar");
  calendar.style.display = "inline-block";
  calendar.innerHTML = svg;
  calendar.scrollLeft = calendar.scrollWidth;
}

function setupUrlSection(url) {
  const svgUrlInput = document.querySelector("input#svg-url");
  svgUrlInput.onclick = (e) => e.target.select();
  svgUrlInput.value = url.href;

  const markdownInput = document.querySelector("input#markdown");
  markdownInput.onclick = (e) => e.target.select();
  markdownInput.value = `[![Contribution activity calendar](${url.href})](${location.href})`;
}

const url = constructUrl();
setupUrlSection(url);
fetchData(url).catch(console.error);
