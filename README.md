# Commitoria

A tool for aggregating and visualising contribution activity from multiple sources.

[![Contribution activity calendar](https://commitoria.thomaszahner.ch/api/calendar.svg?gitlab=thomas-zahner&github=thomas-zahner&font_size=14&cell_size=20&colour_strategy=InterpolationStrategy&inactive_colour=%23f6f5f4&active_colour=%23c061cb)](https://commitoria.thomaszahner.ch/calendar?gitlab=thomas-zahner&github=thomas-zahner&font_size=14&cell_size=20&colour_strategy=InterpolationStrategy&inactive_colour=%23f6f5f4&active_colour=%23c061cb)

✔️ GitHub

✔️ GitLab

✔️ Gitea (Forgejo, Codeberg, ...)

✔️ Bare Git repositories

## Why?

GitHub, GitLab, Gitea, Forgejo, Bitbucket, ...
The list of hosting solutions for your Git repositories seems endless.
It's hard to chose the right platform which is why most people use multiple platforms.
You don't always need a fancy frontend, you might host or use some repositories with bare Git.

One thing most fancy hosting solutions have in common is a way to visualise your contribution activity.
But this visualisation is always limited to the specific platform.
Commitoria enables aggregation and visualisation across multiple platforms,
allowing you to see the bigger picture.
A picture that is closer to reality.

## State

The supported platforms can be found at the very top of the README.
As of now, everything that is aggregated is the contribution count per day.
This data is visualised as an activity calendar (also called contribution graph).

In the future we might want to aggregate and visualise additional activites, such as pull/merge requests.

## Visualisation

The visualisation is roughly a rewrite of
[GitLab's calendar](https://gitlab.com/gitlab-org/gitlab/-/blob/master/app/assets/javascripts/pages/users/activity_calendar.js)
into Rust.
The calendar is rendered as an SVG, which makes it a portable, standalone component.
The rewrite into Rust allows the SVG for example to be rendered by a web server.

## Web

It can make sense to provide this data aggregation and visualisation as a service.
This is what has been done in the [web direcotry](./web).
The library functions are exposed as a HTTP web server so users can utilise the library with REST API calls.

### Web demo

The repository has been deployed to [commitoria.thomaszahner.ch](https://commitoria.thomaszahner.ch).
This allows anybody to create their customised activity calendar.
You can easily include the SVG into your README or website, as illustrated in the beginning of this README.

## Data aggregation

### Unauthenticated GitLab

GitLab provides a neat and simple REST API endpoint returning the activity as JSON object.
See [gitlab.rs](./lib/src/provider/gitlab.rs) for the implementation.

In the future we might want to incorporate the following endpoint to aggregate and visualise additional data:
`GET https://gitlab.com/users/thomas-zahner/calendar_activities?date=2023-11-10`

### Unauthenticated GitHub

Unfortunately, GitHub requires authentication for their REST/GraphQL API endpoints.
Notably they provide a GraphQL endpoint for contribution activity: https://docs.github.com/en/graphql/reference/objects#contributionscollection
However, users might want to be able to skip authentication if possible.

This is why we'll be using the contribution calendar graph directly, which is publicly displayed on all profiles (unless disabled).
Using this approach is "hacky" because we need to extract data from an undocumented endpoint which might change without notice in the future,
but this allows us to skip authentication.

See [github.rs](./lib/src/provider/github.rs) for the implementation.

## Development

Use [Cargo](https://doc.rust-lang.org/cargo/) for development and testing.
With Nix the following commands are supported:

```bash
nix develop
nix build
nix run
```

Additionally, creating a Docker image is as easy as:

```bash
nix build '.#docker'
docker load < ./result
docker run -p 3000:3000 <IMAGE-FROM-ABOVE-STEP>
```
