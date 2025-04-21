const repositories = document.querySelector("#repositories");

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

function addRepositoryLine() {
  const repository = document.createElement("div");

  const username = document.createElement("input");
  username.setAttribute("class", "username");
  username.setAttribute("required", "");
  username.setAttribute("placeholder", "Author's name or email");

  const url = document.createElement("input");
  url.setAttribute("class", "url");
  url.setAttribute("type", "url");
  url.setAttribute("required", "");
  url.setAttribute("placeholder", "URL to Git repository");

  const deleteButton = document.createElement("button");
  deleteButton.textContent = "X";
  deleteButton.setAttribute("title", "Remove repository");
  deleteButton.setAttribute("type", "button");
  deleteButton.onclick = () => repository.remove();

  repository.appendChild(username);
  repository.appendChild(url);
  repository.appendChild(deleteButton);

  repositories.appendChild(repository);
}

function onSubmit(event) {
  event.preventDefault();

  const form = event.target;
  const formData = new FormData(form);
  const params = new URLSearchParams();

  for (const [key, value] of formData.entries()) {
    if (value) {
      params.append(key, value);
    }
  }

  for (const repository of repositories.children) {
    const user_name = repository.querySelector(".username").value;
    const url = repository.querySelector(".url").value;
    params.append("bare_repository", JSON.stringify({ user_name, url }));
  }

  window.location = "/calendar?" + params.toString();
}
