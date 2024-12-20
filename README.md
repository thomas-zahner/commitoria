A tool for aggregating and visualising contribution activity from multiple sources.

Data sources:

- GitHub
- GitLab
- Git repositories by URL

# Data extraction

## Unauthenticated GitLab

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

## Unauthenticated GitHub

Unfortunately, GitHub requires authentication for their REST/GraphQL API endpoints.
Notably they provide a GraphQL endpoint for contribution activity: https://docs.github.com/en/graphql/reference/objects#contributionscollection
However, users might want to be able to skip authentication if possible.

This is why we'll be using the contribution calendar graph directly, which is publicly disaplyed on all profiles (unless disabled).
Using this approach might be "hacky" because we need to extract data from an undocumented endpoint which might change without notice in the future,
but this allows us to skip authentication.

Approach:

curl https://github.com/users/thomas-zahner/contributions

Extract tooltip data with for="contribution-day-component-MOTH-DAY"

# Visualisation

TODO. Potential visualisation inspiration: https://github.com/Platane/snk
