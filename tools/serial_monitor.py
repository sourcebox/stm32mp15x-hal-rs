#!/usr/bin/env python

"""
Simple serial monitor CLI
"""

__author__ = "Oliver Rockstedt <info@sourcebox.de>"
__license__ = "MIT"


# Standard library imports
import argparse
import signal
import sys
import time

# Third-party imports
try:
    import serial
except ImportError:
    print("Modul serial not found.")
    exit()


def parse_args():
    """
    Parse command line arguments

    Returns:
        argparse.Namespace: Parsed arguments
    """
    if len(sys.argv) < 2:
        # Show help when no arguments are given
        sys.argv.append('-h')

    parser = argparse.ArgumentParser(description=__doc__)

    parser.add_argument(
        'port',
        type=str,
        help='Serial port'
    )

    return parser.parse_args()


def connect(port):
    """
    Init serial port and check connectivity

    Parameters:
        port (str): Port name

    Returns:
        Serial:     Instance of Serial class
    """
    try:
        ser = serial.Serial(
            port, 115200, parity=serial.PARITY_NONE, timeout=0.1)
    except serial.serialutil.SerialException as e:
        print(e)
        exit()

    return ser


def main():
    # Exit on Ctrl-C
    signal.signal(signal.SIGINT, lambda signal, frame: (print(), sys.exit(0)))

    args = parse_args()

    ser = connect(args.port)

    print("Listening on %s. Press Ctrl-C to exit." % args.port)

    while True:
        for line in ser.readlines():
            print(line.decode("utf-8", "replace"), end="", flush=True)
        time.sleep(0.01)


if __name__ == '__main__':
    main()
