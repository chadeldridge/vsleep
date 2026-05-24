# vsleep
A very verbose visualizer for sleep.

Sometimes I need to sleep but I need a visual indicator the sleep is still running.

## Installation
Release packages can be found [here](https://github.com/chadeldridge/vsleep/releases).

## Help
```
  -h, --help                       Print help
  -V, --version                    Print version
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
▰▱▱▱▱▱▱
▰▰▱▱▱▱▱
▰▰▰▱▱▱▱
▰▰▰▰▱▱▱
▰▰▰▰▰▱▱
Done

❯ vsleep -v 5; echo "Done"
▰▱▱▱▱▱▱ 5
▰▰▱▱▱▱▱ 4
▰▰▰▱▱▱▱ 3
▰▰▰▰▱▱▱ 2
▰▰▰▰▰▱▱ 1
Done

❯ vsleep -vv 5; echo "Done"
2026-05-23 16:24:22 -0400 ▰▱▱▱▱▱▱ 5
2026-05-23 16:24:23 -0400 ▰▰▱▱▱▱▱ 4
2026-05-23 16:24:24 -0400 ▰▰▰▱▱▱▱ 3
2026-05-23 16:24:25 -0400 ▰▰▰▰▱▱▱ 2
2026-05-23 16:24:26 -0400 ▰▰▰▰▰▱▱ 1
Done
```

## Contributing
If you would like to contribute, see [here](CONTRIBUTING.md).