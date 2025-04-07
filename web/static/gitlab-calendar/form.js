function colourStrategyChange(e) {
  const colourStrategy = document.querySelector("select#colour_strategy");
  const inactiveColour = document.querySelector("#inactive_colour");
  const activeColour = document.querySelector("#active_colour");

  const inactiveColourInput = inactiveColour.querySelector("input");
  const activeColourInput = activeColour.querySelector("input");

  if (colourStrategy.value === "InterpolationStrategy") {
    inactiveColour.style.display = "unset";
    activeColour.style.display = "unset";

    inactiveColourInput.setAttribute("name", "inactive_colour");
    activeColourInput.setAttribute("name", "active_colour");
  } else {
    inactiveColour.style.display = "none";
    activeColour.style.display = "none";

    inactiveColourInput.removeAttribute("name");
    activeColourInput.removeAttribute("name");
  }
}
