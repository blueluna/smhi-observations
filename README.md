# SMHI weather observations

Helpers for the Swedish Meteorological and Hydrological Institute (SMHI) open API for weather observations.

## Example

Get stations that provide the parameter ***temperature***.

```shell
$ cargo run --example get-stations -- --parameter temperature
```

Get temperature readings for the last day from station 97400 (Stockholm-Arlanda).

```shell
$ cargo run --example get-stations -- --parameter temperature --station 97400 --period day
```
