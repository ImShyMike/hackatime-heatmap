# Hackatime Heatmap

Beautiful activity heatmap for your profile!

<a href="https://heatmap.shymike.dev?id=263&standalone=true" title="Click to view detailed data for each day!">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://heatmap.shymike.dev?id=263&theme=dark">
        <img alt="Hackatime activity heatmap." src="https://heatmap.shymike.dev?id=263&theme=light">
    </picture>
</a>

---

## Usage

You eitehr use the color sheme aware version:

```html
<a href="https://heatmap.shymike.dev?id=YOUR_ID&standalone=true" title="Click to view detailed data for each day!">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://heatmap.shymike.dev?id=YOUR_ID&theme=dark">
        <img alt="Hackatime activity heatmap." src="https://heatmap.shymike.dev?id=YOUR_ID&theme=light">
    </picture>
</a>
```

Or the simple version:

```html
<img alt="Hackatime activity heatmap." title="Click to view detailed data for each day!" src="https://heatmap.shymike.dev?id=YOUR_ID">
```

## Configuration

All the parameters that can be adjusted are:

- [Id](#id)
- [Timezone](#timezone)
- [Theme](#theme)
- [Padding](#padding)
- [Rounding](#rounding)
- [Cell Size](#cell-size)
- [Ranges](#ranges)
- [Standalone](#standalone)

Some example are:

- [https://heatmap.shymike.dev?id=1&padding=2&theme=catppuccin_dark&timezone=EST&rounding=100](https://heatmap.shymike.dev?id=1&padding=2&theme=catppuccin_dark&timezone=EST&rounding=100)
    ![Hackatime heatmap with Catppuccin dark theme, EST timezone, 100% rounding, and 2px padding](https://heatmap.shymike.dev?id=1&padding=2&theme=catppuccin_dark&timezone=EST&rounding=100)
- [https://heatmap.shymike.dev?id=1&padding=0&theme=light&timezone=America/Los_Angeles&rounding=0](https://heatmap.shymike.dev?id=1&padding=0&theme=light&timezone=America/Los_Angeles&rounding=0)
    ![Hackatime heatmap with light theme, America/Los_Angeles timezone, 0% rounding, and 0px padding](https://heatmap.shymike.dev?id=1&padding=0&theme=light&timezone=America/Los_Angeles&rounding=0)

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

### Padding

The padding around each cell in pixels. Defaults to `2`.

- [https://heatmap.shymike.dev?id=1&padding=5](https://heatmap.shymike.dev?id=1&padding=5)
    ![Hackatime heatmap with 5px padding](https://heatmap.shymike.dev?id=1&padding=5)
- [https://heatmap.shymike.dev?id=1&padding=0](https://heatmap.shymike.dev?id=1&padding=0)
    ![Hackatime heatmap with 0px padding](https://heatmap.shymike.dev?id=1&padding=0)

### Rounding

The rounding percentage of each cell. Defaults to `50` %.

- [https://heatmap.shymike.dev?id=1&rounding=0](https://heatmap.shymike.dev?id=1&rounding=0)
    ![Hackatime heatmap with 0% rounding](https://heatmap.shymike.dev?id=1&rounding=0)
- [https://heatmap.shymike.dev?id=1&rounding=100](https://heatmap.shymike.dev?id=1&rounding=100)
    ![Hackatime heatmap with 100% rounding](https://heatmap.shymike.dev?id=1&rounding=100)

### Cell Size

The size of each cell in pixels. Defaults to `15` px.

- [https://heatmap.shymike.dev?id=1&cell_size=10](https://heatmap.shymike.dev?id=1&cell_size=10)
    ![Hackatime heatmap with 5px cell size](https://heatmap.shymike.dev?id=1&cell_size=10)
- [https://heatmap.shymike.dev?id=1&cell_size=25](https://heatmap.shymike.dev?id=1&cell_size=25)
    ![Hackatime heatmap with 25px cell size](https://heatmap.shymike.dev?id=1&cell_size=25)

### Ranges

The percentage ranges for each color as a 3 item string list. Defaults to `70,30,10`.

- [https://heatmap.shymike.dev?id=1&ranges=80,50,20](https://heatmap.shymike.dev?id=1&ranges=80,50,20)
    ![Hackatime heatmap with ranges 80,50,20](https://heatmap.shymike.dev?id=1&ranges=80,50,20)
- [https://heatmap.shymike.dev?id=1&ranges=75,50,25](https://heatmap.shymike.dev?id=1&ranges=75,50,25)
    ![Hackatime heatmap with ranges 75,50,25](https://heatmap.shymike.dev?id=1&ranges=75,50,25)

### Standalone

Whether or not to embed HTML into the request. Defaults to `false`

- [https://heatmap.shymike.dev?id=1&standalone=true](https://heatmap.shymike.dev?id=1&standalone=true)
- [https://heatmap.shymike.dev?id=1&standalone=false](https://heatmap.shymike.dev?id=1&standalone=false)
