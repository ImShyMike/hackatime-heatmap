# Hackatime Heatmap

Easy to set up [Hackatime](https://hackatime.hackclub.com) coding activity heatmap for your profile!

<a href="https://heatmap.shymike.dev?id=263&standalone=true" title="Click to view detailed data for each day!">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://heatmap.shymike.dev?id=263&theme=dark">
        <img alt="Hackatime activity heatmap." src="https://heatmap.shymike.dev?id=263&theme=light">
    </picture>
</a>

<sub>^^^ Those are my stats!</sub>

### [Make and customize your own here!](https://hackatime-heatmap.shymike.dev)

---

## Usage

You can either use the color scheme aware version:

```html
<a href="https://heatmap.shymike.dev?id=YOUR_ID&standalone=true" title="Click to view detailed data for each day!">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://heatmap.shymike.dev?id=YOUR_ID&theme=dark">
        <img alt="Hackatime activity heatmap." src="https://heatmap.shymike.dev?id=YOUR_ID&theme=light">
    </picture>
</a>
```

> [!CAUTION]
> Only the top url in the `a` tag can have the `standalone` tag set to `true`, otherwise the graph will not appear!

Or the simple version:

```html
<img alt="Hackatime activity heatmap." title="Click to view detailed data for each day!" src="https://heatmap.shymike.dev?id=YOUR_ID">
```

## Configuration

All the parameters that can be adjusted are:

- [Id](#id)
- [Timezone](#timezone)
- [Theme](#theme)
- [Cell Size](#cell-size)
- [Padding](#padding)
- [Rounding](#rounding)
- [Ranges](#ranges)
- [Year](#year)
- [Show Labels](#show-labels)
- [Standalone](#standalone)

Some examples:

- [https://heatmap.shymike.dev?id=1&theme=catppuccin_dark&timezone=EST](https://heatmap.shymike.dev?id=1&theme=catppuccin_dark&timezone=EST)
    ![Hackatime heatmap with Catppuccin dark theme, EST timezone](https://heatmap.shymike.dev?id=1&theme=catppuccin_dark&timezone=EST)
- [https://heatmap.shymike.dev?id=1&theme=light&timezone=America/Los_Angeles](https://heatmap.shymike.dev?id=1&theme=light&timezone=America/Los_Angeles)
    ![Hackatime heatmap with light theme, America/Los_Angeles timezone](https://heatmap.shymike.dev?id=1&theme=light&timezone=America/Los_Angeles)

### Id

**This parameter is mandatory** and can be set to your Hackatime ID, Hackatime Username or Slack ID

Examples:

- [https://heatmap.shymike.dev?id=1](https://heatmap.shymike.dev?id=1)
    ![Hackatime heatmap for ID 1](https://heatmap.shymike.dev?id=1)
- [https://heatmap.shymike.dev?id=msw](https://heatmap.shymike.dev?id=msw)
    ![Hackatime heatmap for user msw](https://heatmap.shymike.dev?id=msw)
- [https://heatmap.shymike.dev?id=U0C7B14Q3](https://heatmap.shymike.dev?id=U0C7B14Q3)
    ![Hackatime heatmap for Slack ID U0C7B14Q3](https://heatmap.shymike.dev?id=U0C7B14Q3)

### Timezone

This parameter can be set to the identifier of any timezone on the [tz database](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones). Defaults to `UTC`.

- [https://heatmap.shymike.dev?id=1&timezone=America/Los_Angeles](https://heatmap.shymike.dev?id=1&timezone=America/Los_Angeles)
    ![Hackatime heatmap in America/Los_Angeles timezone](https://heatmap.shymike.dev?id=1&timezone=America/Los_Angeles)
- [https://heatmap.shymike.dev?id=1&timezone=EST](https://heatmap.shymike.dev?id=1&timezone=EST)
    ![Hackatime heatmap in EST timezone](https://heatmap.shymike.dev?id=1&timezone=EST)

### Theme

The theme can be set to either `dark`, `light`, `catppuccin_dark` or `catppuccin_light`. Defaults to `dark`.

- [https://heatmap.shymike.dev?id=1&theme=dark](https://heatmap.shymike.dev?id=1&theme=dark)
    ![Hackatime heatmap with dark theme](https://heatmap.shymike.dev?id=1&theme=dark)
- [https://heatmap.shymike.dev?id=1&theme=light](https://heatmap.shymike.dev?id=1&theme=light)
    ![Hackatime heatmap with light theme](https://heatmap.shymike.dev?id=1&theme=light)
- [https://heatmap.shymike.dev?id=1&theme=catppuccin_dark](https://heatmap.shymike.dev?id=1&theme=catppuccin_dark)
    ![Hackatime heatmap with Catppuccin dark theme](https://heatmap.shymike.dev?id=1&theme=catppuccin_dark)
- [https://heatmap.shymike.dev?id=1&theme=catppuccin_light](https://heatmap.shymike.dev?id=1&theme=catppuccin_light)
    ![Hackatime heatmap with Catppuccin light theme](https://heatmap.shymike.dev?id=1&theme=catppuccin_light)

### Cell Size

The size of each cell in pixels. Defaults to `10` px.

- [https://heatmap.shymike.dev?id=1&cell_size=8](https://heatmap.shymike.dev?id=1&cell_size=8)
    ![Hackatime heatmap with 8px cell size](https://heatmap.shymike.dev?id=1&cell_size=8)
- [https://heatmap.shymike.dev?id=1&cell_size=15](https://heatmap.shymike.dev?id=1&cell_size=15)
    ![Hackatime heatmap with 15px cell size](https://heatmap.shymike.dev?id=1&cell_size=15)

### Padding

The padding between each cell in pixels. Defaults to `3`.

- [https://heatmap.shymike.dev?id=1&padding=1](https://heatmap.shymike.dev?id=1&padding=1)
    ![Hackatime heatmap with 1px padding](https://heatmap.shymike.dev?id=1&padding=1)
- [https://heatmap.shymike.dev?id=1&padding=5](https://heatmap.shymike.dev?id=1&padding=5)
    ![Hackatime heatmap with 5px padding](https://heatmap.shymike.dev?id=1&padding=5)

### Rounding

The rounding percentage of each cell (0-100). Defaults to `20`.

- [https://heatmap.shymike.dev?id=1&rounding=0](https://heatmap.shymike.dev?id=1&rounding=0)
    ![Hackatime heatmap with 0% rounding](https://heatmap.shymike.dev?id=1&rounding=0)
- [https://heatmap.shymike.dev?id=1&rounding=100](https://heatmap.shymike.dev?id=1&rounding=100)
    ![Hackatime heatmap with 100% rounding](https://heatmap.shymike.dev?id=1&rounding=100)

### Ranges

The percentage ranges for each color as a 3 item string list. Defaults to `70,30,10`.

- [https://heatmap.shymike.dev?id=1&ranges=80,50,20](https://heatmap.shymike.dev?id=1&ranges=80,50,20)
    ![Hackatime heatmap with ranges 80,50,20](https://heatmap.shymike.dev?id=1&ranges=80,50,20)
- [https://heatmap.shymike.dev?id=1&ranges=75,50,25](https://heatmap.shymike.dev?id=1&ranges=75,50,25)
    ![Hackatime heatmap with ranges 75,50,25](https://heatmap.shymike.dev?id=1&ranges=75,50,25)

### Year

Display data for a specific calendar year (Jan 1 - Dec 31) instead of the last 365 days. Use `current` for the current year, or specify a year number. If not set, shows the last 365 days.

- [https://heatmap.shymike.dev?id=1&year=current](https://heatmap.shymike.dev?id=1&year=current)
    ![Hackatime heatmap for current year](https://heatmap.shymike.dev?id=1&year=current)
- [https://heatmap.shymike.dev?id=1&year=2025](https://heatmap.shymike.dev?id=1&year=2025)
    ![Hackatime heatmap for 2025](https://heatmap.shymike.dev?id=1&year=2025)
- [https://heatmap.shymike.dev?id=1&year=2026](https://heatmap.shymike.dev?id=1&year=2026)
    ![Hackatime heatmap for 2026](https://heatmap.shymike.dev?id=1&year=2026)

### Show Labels

Whether to show month labels, weekday labels (Mon/Wed/Fri), and the "Less/More" legend. Defaults to `true`.

- [https://heatmap.shymike.dev?id=1&show_labels=true](https://heatmap.shymike.dev?id=1&show_labels=true)
    ![Hackatime heatmap with labels](https://heatmap.shymike.dev?id=1&show_labels=true)
- [https://heatmap.shymike.dev?id=1&show_labels=false](https://heatmap.shymike.dev?id=1&show_labels=false)
    ![Hackatime heatmap without labels](https://heatmap.shymike.dev?id=1&show_labels=false)

### Standalone

Whether or not to embed HTML into the request. Defaults to `false`

- [https://heatmap.shymike.dev?id=1&standalone=true](https://heatmap.shymike.dev?id=1&standalone=true)
- [https://heatmap.shymike.dev?id=1&standalone=false](https://heatmap.shymike.dev?id=1&standalone=false)
