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

- [https://heatmap.shymike.dev?id=1&labels=true](https://heatmap.shymike.dev?id=1&labels=true)
    ![Hackatime heatmap with labels](https://heatmap.shymike.dev?id=1&labels=true)
- [https://heatmap.shymike.dev?id=1&labels=false](https://heatmap.shymike.dev?id=1&labels=false)
    ![Hackatime heatmap without labels](https://heatmap.shymike.dev?id=1&labels=false)


---

## Snake Animation

Generate an animated SVG where a snake navigates through your coding activity and consumes your heatmap dots! Available at the `/snake` endpoint.

### Snake Usage

You can use the color scheme aware version:

```html
<picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://heatmap.shymike.dev/snake?id=YOUR_ID&theme=dark">
    <img alt="Hackatime snake animation" src="https://heatmap.shymike.dev/snake?id=YOUR_ID&theme=light">
</picture>
```

Or the automatic theme detection version:

```html
<img alt="Hackatime snake animation" title="Hackatime snake animation" src="https://heatmap.shymike.dev/snake?id=YOUR_ID&theme=auto">
```

Or the simple version with default dark theme:

```html
<img alt="Hackatime snake animation" title="Hackatime snake animation" src="https://heatmap.shymike.dev/snake?id=YOUR_ID">
```

### Snake Configuration

All parameters that can be adjusted for the `/snake` endpoint are:

- [Id](#id)
- [Theme](#snake-theme)
- [Timezone](#timezone)
- [Cell Size](#snake-cell-size)
- [Dot Size](#dot-size)
- [Speed](#speed)
- [Snake Color](#snake-color)
- [Ranges](#ranges)
- [Year](#year)

Some examples:

- [https://heatmap.shymike.dev/snake?id=1&theme=dark](https://heatmap.shymike.dev/snake?id=1&theme=dark)
    ![Hackatime snake animation with dark theme](https://heatmap.shymike.dev/snake?id=1&theme=dark)
- [https://heatmap.shymike.dev/snake?id=1&theme=catppuccin_dark&snake_color=#10b981](https://heatmap.shymike.dev/snake?id=1&theme=catppuccin_dark&snake_color=#10b981)
    ![Hackatime snake animation with Catppuccin dark theme and green snake](https://heatmap.shymike.dev/snake?id=1&theme=catppuccin_dark&snake_color=#10b981)
- [https://heatmap.shymike.dev/snake?id=1&theme=light&speed=80](https://heatmap.shymike.dev/snake?id=1&theme=light&speed=80)
    ![Hackatime snake animation with light theme and faster speed](https://heatmap.shymike.dev/snake?id=1&theme=light&speed=80)

#### Snake Theme

The theme can be set to fixed themes or adaptive themes that react to system `prefers-color-scheme`:

- `dark`: Forced dark theme (default).
- `light`: Forced light theme.
- `catppuccin_dark`: Forced Catppuccin Mocha theme.
- `catppuccin_light`: Forced Catppuccin Latte theme.
- `auto`: Automatically adapts between GitHub Light and GitHub Dark based on user system preference.
- `catppuccin`: Automatically adapts between Catppuccin Latte and Catppuccin Mocha based on user system preference.

Examples:

- [https://heatmap.shymike.dev/snake?id=1&theme=light](https://heatmap.shymike.dev/snake?id=1&theme=light)
    ![Hackatime snake animation with light theme](https://heatmap.shymike.dev/snake?id=1&theme=light)
- [https://heatmap.shymike.dev/snake?id=1&theme=auto](https://heatmap.shymike.dev/snake?id=1&theme=auto)
    ![Hackatime snake animation with auto theme](https://heatmap.shymike.dev/snake?id=1&theme=auto)
- [https://heatmap.shymike.dev/snake?id=1&theme=catppuccin](https://heatmap.shymike.dev/snake?id=1&theme=catppuccin)
    ![Hackatime snake animation with catppuccin theme](https://heatmap.shymike.dev/snake?id=1&theme=catppuccin)

#### Snake Cell Size

The size of each cell in pixels. Defaults to `14` px for the snake animation.

- [https://heatmap.shymike.dev/snake?id=1&cell_size=12](https://heatmap.shymike.dev/snake?id=1&cell_size=12)
    ![Hackatime snake animation with 12px cell size](https://heatmap.shymike.dev/snake?id=1&cell_size=12)
- [https://heatmap.shymike.dev/snake?id=1&cell_size=16](https://heatmap.shymike.dev/snake?id=1&cell_size=16)
    ![Hackatime snake animation with 16px cell size](https://heatmap.shymike.dev/snake?id=1&cell_size=16)

#### Dot Size

The size of the activity dots in pixels. Defaults to `75%` of `cell_size` (`10.5` px when `cell_size` is `14`).

- [https://heatmap.shymike.dev/snake?id=1&dot_size=8](https://heatmap.shymike.dev/snake?id=1&dot_size=8)
    ![Hackatime snake animation with 8px dot size](https://heatmap.shymike.dev/snake?id=1&dot_size=8)
- [https://heatmap.shymike.dev/snake?id=1&dot_size=12](https://heatmap.shymike.dev/snake?id=1&dot_size=12)
    ![Hackatime snake animation with 12px dot size](https://heatmap.shymike.dev/snake?id=1&dot_size=12)

#### Speed

The duration of each animation step in milliseconds. Smaller values make the snake move faster. Defaults to `120` ms.

- [https://heatmap.shymike.dev/snake?id=1&speed=80](https://heatmap.shymike.dev/snake?id=1&speed=80)
    ![Hackatime snake animation with 80ms speed](https://heatmap.shymike.dev/snake?id=1&speed=80)
- [https://heatmap.shymike.dev/snake?id=1&speed=200](https://heatmap.shymike.dev/snake?id=1&speed=200)
    ![Hackatime snake animation with 200ms speed](https://heatmap.shymike.dev/snake?id=1&speed=200)

#### Snake Color

The color of the snake in hex format. Defaults to `#7c3aed`.

- [https://heatmap.shymike.dev/snake?id=1&snake_color=#10b981](https://heatmap.shymike.dev/snake?id=1&snake_color=#10b981)
    ![Hackatime snake animation with green snake](https://heatmap.shymike.dev/snake?id=1&snake_color=#10b981)
- [https://heatmap.shymike.dev/snake?id=1&snake_color=#f59e0b](https://heatmap.shymike.dev/snake?id=1&snake_color=#f59e0b)
    ![Hackatime snake animation with amber snake](https://heatmap.shymike.dev/snake?id=1&snake_color=#f59e0b)

### Standalone

Whether or not to embed HTML into the request. Defaults to `false`

- [https://heatmap.shymike.dev?id=1&standalone=true](https://heatmap.shymike.dev?id=1&standalone=true)
- [https://heatmap.shymike.dev?id=1&standalone=false](https://heatmap.shymike.dev?id=1&standalone=false)
