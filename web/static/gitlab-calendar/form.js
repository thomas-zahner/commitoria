function colourStrategyChange(e) {
  const colourStrategy = document.querySelector("select#colour_strategy");
  const inactiveColour = document.querySelector("input#inactive_colour");
  const activeColour = document.querySelector("input#active_colour");

  if (colourStrategy.value === "InterpolationStrategy") {
    inactiveColour.removeAttribute("hidden");
    activeColour.removeAttribute("hidden");

    inactiveColour.setAttribute("name", "inactive_colour");
    activeColour.setAttribute("name", "active_colour");
  } else {
    inactiveColour.setAttribute("hidden", "");
    activeColour.setAttribute("hidden", "");
  }
}
