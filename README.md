# dosh

A terminal dashboard for tracking cryptocurrency prices.

![A preview of the dashboard](preview.png)

## Usage

Running the binary `dosh` with no arguments is equivalent to the following:

`dosh coin=bitcoin quantity=1 currency=usd days=7`

All settings are in the form `x=y`. Below is a list of all valid options:

- `coin` is the name of the cryptocurrency to track, as its full name.
- `quantity` is the amount of a cryptocurrency to use in exchange conversions, as a decimal value.
- `currency` is the name of the primary currency to compare against, as a three-letter currency code.
- `days` is the number of days to show historical data and percentage change for, as an integer.