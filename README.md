# Hackatime Heatmap

Beautiful activity heatmap for your profile!

<a href="https://heatmap.shymike.dev?id=263" title="Click to view detailed data for each day!">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://heatmap.shymike.dev?id=263&theme=dark">
        <img alt="Hackatime activity heatmap." src="https://heatmap.shymike.dev?id=263&theme=light">
    </picture>
</a>

---

## Usage

You eitehr use the color sheme aware version:

```html
<a href="https://heatmap.shymike.dev?id=YOUR_ID" title="Click to view detailed data for each day!">
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

Some example are:

- [https://heatmap.shymike.dev?id=1&padding=0&theme=catppuccin_dark&timezone=EST&rounding=100](https://heatmap.shymike.dev?id=1&padding=0&theme=catppuccin_dark&timezone=EST&rounding=100)
- [https://heatmap.shymike.dev?id=1&padding=0&theme=light&timezone=America/Los_Angeles&rounding=0](https://heatmap.shymike.dev?id=1&padding=0&theme=light&timezone=America/Los_Angeles&rounding=0)

### Id

**This parameter is mandatory** and can be set to your Hackatime ID, Hackatime Username or Slack ID

Examples:

- [https://heatmap.shymike.dev?id=1](https://heatmap.shymike.dev?id=1)
- [https://heatmap.shymike.dev?id=msw](https://heatmap.shymike.dev?id=msw)
- [https://heatmap.shymike.dev?id=U0C7B14Q3](https://heatmap.shymike.dev?id=U0C7B14Q3)

### Timezone

This parameter can be set to the identifier of any timezone on the [tz database](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones). Defaults to `UTC`.

- [https://heatmap.shymike.dev?id=1&timezone=America/Los_Angeles](https://heatmap.shymike.dev?id=1&timezone=America/Los_Angeles)
- [https://heatmap.shymike.dev?id=1&timezone=EST](https://heatmap.shymike.dev?id=1&timezone=EST)

### Theme

The theme can be set to either `dark`, `light`, `catppuccin_dark` or `catppuccin_light`. Defaults to `dark`.

- [https://heatmap.shymike.dev?id=1&theme=dark](https://heatmap.shymike.dev?id=1&theme=dark)
- [https://heatmap.shymike.dev?id=1&theme=light](https://heatmap.shymike.dev?id=1&theme=light)
- [https://heatmap.shymike.dev?id=1&theme=catppuccin_dark](https://heatmap.shymike.dev?id=1&theme=catppuccin_dark)
- [https://heatmap.shymike.dev?id=1&theme=catppuccin_light](https://heatmap.shymike.dev?id=1&theme=catppuccin_light)

### Padding

The padding around each cell in pixels. Defaults to `2`.

- [https://heatmap.shymike.dev?id=1&padding=5](https://heatmap.shymike.dev?id=1&padding=5)
- [https://heatmap.shymike.dev?id=1&padding=0](https://heatmap.shymike.dev?id=1&padding=0)

### Rounding

The rounding percentage of each cell. Defaults to `50` %.

- [https://heatmap.shymike.dev?id=1&rounding=0](https://heatmap.shymike.dev?id=1&rounding=0)
- [https://heatmap.shymike.dev?id=1&rounding=100](https://heatmap.shymike.dev?id=1&rounding=100)

### Cell Size

The size of each cell in pixels. Defaults to `15` px.

- [https://heatmap.shymike.dev?id=1&cell_size=5](https://heatmap.shymike.dev?id=1&cell_size=5)
- [https://heatmap.shymike.dev?id=1&cell_size=25](https://heatmap.shymike.dev?id=1&cell_size=25)

### Ranges

The percentage ranges for each color as a 3 item string list. Defaults to `70,30,10`.

- [https://heatmap.shymike.dev?id=1&ranges=80,50,20](https://heatmap.shymike.dev?id=1&ranges=80,50,20)
- [https://heatmap.shymike.dev?id=1&ranges=75,50,25](https://heatmap.shymike.dev?id=1&ranges=75,50,25)
