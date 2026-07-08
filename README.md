# haytime

Date and time manipulation plugin for Hayashi.

## Installation

```bash
hay install sheep-farm/haytime
```

## Usage

```hayashi
import("sheep-farm/haytime", as=ht)

// Parse a date string
let ts = ht::parse_date("2024-01-15", "%Y-%m-%d")
print(ts)

// Get current timestamp
let now = ht::now()
print(now)

// Add days
let future = ht::add_days(now, 30)
print(future)

// Extract components
let y = ht::year(now)
let m = ht::month(now)
let d = ht::day(now)
print("Year:", y, "Month:", m, "Day:", d)

// Format date
let formatted = ht::format_date(now, "%Y-%m-%d")
print(formatted)

// Check if weekend
let is_weekend = ht::is_weekend(now)
print("Is weekend:", is_weekend)

// Get start of month
let month_start = ht::start_of_month(now)
print("Month start:", month_start)
```

## Functions

### Parsing and Formatting
- `parse_date(date_str, format)` - Parse date string to timestamp
- `format_date(timestamp, format)` - Format timestamp to string
- `now()` - Current timestamp in milliseconds

### Arithmetic
- `add_days(timestamp, days)` - Add days to timestamp
- `add_months(timestamp, months)` - Add months to timestamp
- `add_years(timestamp, years)` - Add years to timestamp
- `diff_days(timestamp1, timestamp2)` - Difference in days

### Extraction
- `year(timestamp)` - Extract year
- `month(timestamp)` - Extract month (1-12)
- `day(timestamp)` - Extract day (1-31)
- `hour(timestamp)` - Extract hour (0-23)
- `minute(timestamp)` - Extract minute (0-59)
- `second(timestamp)` - Extract second (0-59)
- `weekday(timestamp)` - Extract weekday (0=Sunday, 6=Saturday)

### Helpers
- `is_weekend(timestamp)` - Check if timestamp is weekend
- `start_of_month(timestamp)` - First day of month at 00:00:00
- `end_of_month(timestamp)` - Last day of month at 23:59:59
- `start_of_year(timestamp)` - First day of year at 00:00:00
- `end_of_year(timestamp)` - Last day of year at 23:59:59
- `days_in_month(timestamp)` - Number of days in month

## Date Format Strings

- `%Y` - Year (4 digits)
- `%m` - Month (01-12)
- `%d` - Day (01-31)
- `%H` - Hour (00-23)
- `%M` - Minute (00-59)
- `%S` - Second (00-59)

Example formats:
- `%Y-%m-%d` - "2024-01-15"
- `%d/%m/%Y` - "15/01/2024"
- `%Y-%m-%d %H:%M:%S` - "2024-01-15 14:30:00"

## Development

```bash
cargo build --release
cp target/release/libhaytime.so ~/.hay/packages/sheep-farm/haytime.so
```
