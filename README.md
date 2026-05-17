# vsleep
A simple app for visualizing sleep. 

Sometimes I need to sleep but I need a visual indicator that the sleep is still running.

## Help
```
A simple app for visualizing sleep.

Usage: vsleep [OPTIONS] [DURATION]

Arguments:
  [DURATION]  Sleep duration in seconds

Options:
  -f, --file <FILE>        Alternate spinners file to import. Expected format: { "spinner_name": { frames: [ "-----", "1----", "12---", "123--", "1234-", "12345", ], } } [default: spinners.json]
      --list               List spinner names
  -s, --spinner <SPINNER>  Name of spinner to use. Default: aesthetic [default: aesthetic]
  -h, --help               Print help
  -V, --version            Print version
```

## Spinners
While not directly used as a spinner I am using a modified version of the spinner file from [cli-spinners](https://github.com/sindresorhus/cli-spinners) as a default visualizer.

You can provide your own spinners by passing the path to a json file with `-f, --file`. The expected format is:
```json
{
  "spinner_name1": {
    "frames": [
      "1----",
      "12---",
      "123--",
      "1234-",
      "12345"
    ]
  },
  "spinner_name2": {
    "frames": [
      "|--",
      "-|-",
      "--|"
    ]
  }
}
```

## Examples
```bash
❯ vsleep 5; echo "Done"
▰▱▱▱▱▱▱ 1
▰▰▱▱▱▱▱ 2
▰▰▰▱▱▱▱ 3
▰▰▰▰▱▱▱ 4
▰▰▰▰▰▱▱ 5
Done
```
