# `shaper_pcb`

A command line tool to generate [Shaper Origin](https://www.shapertools.com/en-us) toolpaths from gerber files.
This can be used to mill PCBs with a Shaper Origin router.

## Example

Below is the PCB for a [Full Adder](https://en.wikipedia.org/wiki/Adder_(electronics)#Full_adder) circuit built with signal relays.

|![](https://github.com/user-attachments/assets/10649f2e-d167-4f4c-9922-45b2840801ec)|![](https://github.com/user-attachments/assets/f4405da9-8fc0-4d94-8ade-05dce0d26d9f)|
|--|--|

## Usage

```
Usage: shaper_pcb [OPTIONS] <INPUT> <OUTPUT>

Arguments:
  <INPUT>   Gerber / Drill file to process
  <OUTPUT>  SVG file to output

Options:
  -o, --outline <OUTLINE>                        Optional outline layer
  -a, --aperture-thickness <APERTURE_THICKNESS>  Aperture radius multiplier [default: 1]
  -t, --trace-thickness <TRACE_THICKNESS>        Trace thickness multiplier [default: 1]
  -p, --pads-only                                Ignore traces, only export apatures
  -h, --help                                     Print help
  -V, --version                                  Print version
```
