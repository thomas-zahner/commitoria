# Commitoria

A tool for aggregating and visualising contribution activity from multiple sources.

## Why?

GitHub, GitLab, Bitbucket, Gitea, Forgejo, ...
The list of hosting solutions for your Git repositories seems endless.
It's hard to chose the right platform which is why most people use multiple platforms.
You don't always need a fancy frontend, you might host or use some repositories with bare Git.

One thing all the fancy hosting solutions have in common is a way to visualise your contribution activity.
But this visualisation is always limited to the specific platform.
Commitoria enables aggregation and visualisation across multiple platforms,
allowing you to see the bigger picture.
A picture that is closer to reality.

## State

Currently, GitHub and GitLab (gitlab.com) are supported as data sources.
The goal is to support more hosting solutions and custom domains.
Bare Git repositories are also not yet supported.

As of now, everything that is aggregated is the contribution count per day.
This data is visualised as an activity calendar (also called contribution graph),
mainly inspired by [GitLab's calendar](https://gitlab.com/gitlab-org/gitlab/-/blob/master/app/assets/javascripts/pages/users/activity_calendar.js).

In the future we might want to aggregate and visualise additional activites, such as pull/merge requests.

## Data aggregation

### Unauthenticated GitLab

GET https://gitlab.com/users/thomas-zahner/calendar.json

```json
{
    "2023-10-06":2,
    "2023-10-13":1,
    "2024-04-24":1,
    "2024-09-17":8,
    "2024-09-22":1
}
```

GET https://gitlab.com/users/thomas-zahner/calendar_activities?date=2023-11-10

Gives HTML of recent activities.

### Unauthenticated GitHub

Unfortunately, GitHub requires authentication for their REST/GraphQL API endpoints.
Notably they provide a GraphQL endpoint for contribution activity: https://docs.github.com/en/graphql/reference/objects#contributionscollection
However, users might want to be able to skip authentication if possible.

This is why we'll be using the contribution calendar graph directly, which is publicly disaplyed on all profiles (unless disabled).
Using this approach is "hacky" because we need to extract data from an undocumented endpoint which might change without notice in the future,
but this allows us to skip authentication.
