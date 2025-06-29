const repositories = document.querySelector("#repositories");

function colourStrategyChange(e) {
  const colourStrategy = document.querySelector("select#colour_strategy");
  const inactiveColour = document.querySelector("#inactive_colour");
  const activeColour = document.querySelector("#active_colour");

  const inactiveColourInput = inactiveColour.querySelector("input");
  const activeColourInput = activeColour.querySelector("input");

  if (colourStrategy.value === "InterpolationStrategy") {
    inactiveColour.classList.remove('hidden');
    activeColour.classList.remove('hidden');

    inactiveColourInput.setAttribute("name", "inactive_colour");
    activeColourInput.setAttribute("name", "active_colour");
  } else {
    inactiveColour.classList.add('hidden');
    activeColour.classList.add('hidden');

    inactiveColourInput.removeAttribute("name");
    activeColourInput.removeAttribute("name");
  }
}

function addRepositoryLine() {
  const repository = document.createElement("div");

  const username = document.createElement("input");
  username.setAttribute("data-form-name", "user_name");
  username.setAttribute("required", "");
  username.setAttribute("placeholder", "Username or email");

  const url = document.createElement("input");
  url.setAttribute("data-form-name", "url");
  url.setAttribute("type", "url");
  url.setAttribute("required", "");
  url.setAttribute("placeholder", "URL to hoster or Git repository");

  const type = document
    .querySelector("#repository-type")
    .content.cloneNode(true);

  const deleteButton = document.createElement("button");
  deleteButton.textContent = "X";
  deleteButton.setAttribute("title", "Remove repository");
  deleteButton.setAttribute("type", "button");
  deleteButton.onclick = () => repository.remove();

  repository.appendChild(username);
  repository.appendChild(url);
  repository.appendChild(type);
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
    const data = {};
    for (const child of repository.children) {
      const value = child.value;
      const name = child.dataset.formName;

      if (value && name) {
        data[name] = value;
      }
    }

    params.append("repositories", JSON.stringify(data));
  }

  window.location = "/calendar?" + params.toString();
}
