# remind

A simple command-line tool that reminds you, via a notification, to do something.

## Usage

```
$ remind DELAY MESSAGE
```

* `DELAY` is the amount of time before you get a notification. It should be a number followed by 's' (for seconds), 'm' (for minutes) or 'h' (for hours).
* `MESSAGE` is an optional message that will be displayed in the notification.

### Examples

A reminder to water the plants in 5 minutes:

```
$ remind 5m water the plants
```

A generic reminder in 1 hour:
```
$ remind 1h
```
