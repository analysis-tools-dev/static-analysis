#############################################################################
# Copyright (C) 2019 LCIS Laboratory - Cyril Bresch
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
#     the Free Software Foundation, in version 3.
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU General Public License for more details.
# You should have received a copy of the GNU General Public License
# along with this program. If not, see <http://www.gnu.org/licenses/>.
# This program is part of the SecPump @https://github.com/r3glisss/SecPump
#############################################################################
import struct
import sys
import signal
import time

from bluepy.btle import Scanner, DefaultDelegate, Peripheral


class ScanDelegate(DefaultDelegate):
    def __init__(self):
        DefaultDelegate.__init__(self)

    def handleDiscovery(self, dev, isNewDev, isNewData):
        if isNewDev:
            print "[*] Discovered device", dev.addr
        elif isNewData:
            print "[*] Received new data from", dev.addr


def FindBLEDevice(devices):
    for dev in devices:
        print "Device %s (%s), RSSI=%d dB" % (dev.addr, dev.addrType, dev.rssi)
        for (adtype, desc, value) in dev.getScanData():
            print "  %s = %s" % (desc, value)
            if "SecPump" in value:
                print "[+] Found Medical Pump"
                return dev


def SignalHandler(sig, frame):
    print('[!] Exiting the Program')
    sys.exit(0)


def ConnectDevice(device):
    print "[*] Connecting to Device..."
    dev = Peripheral(device.addr)
    return dev


def DisconnectDevice(device):
    print "[*] Disconnecting to Device..."
    device.disconnect()


def DisplayServices(device):
    print "[*] Discovering Services..."
    for svc in device.services:
        print svc


def GetPumpService(device, name):
    if name == "Pump":
        print "[*] Accessing Pump Service"
        return device.getServiceByUUID("42821A40-E477-11E2-82D0-0002A5D5C51B")


def GetPumpCharacteristics(service, name):
    if service.uuid == "42821A40-E477-11E2-82D0-0002A5D5C51B":
        for char in service.getCharacteristics():
            if name == "MODE":
                if char.uuid == "CD20C480-E48B-11E2-840B-0002A5D5C51B":
                    print "[*] Getting MODE characteristic"
                    return char

            if name == "BOLUS":
                if char.uuid == "A32E5520-E477-11E2-A9E3-0002A5D5C51B":
                    print "[*] Getting BOLUS characteristic"
                    return char


def WriteCharacteristic(charac, msg):
    charac.write(msg, True)


def BLEWriter(charac):
    msg = raw_input("Msg >")
    WriteCharacteristic(charac, msg)


def main(argv):
    print "[*] Scanning For Device"
    signal.signal(signal.SIGINT, SignalHandler)

    scanner = Scanner().withDelegate(ScanDelegate())
    # Fedora 30 bug fixe
    devices = scanner.scan(10.0, passive=True)

    device = FindBLEDevice(devices)
    deviceConnect = ConnectDevice(device)
    DisplayServices(deviceConnect)
    # Getting Characteristics from Command service
    service = GetPumpService(deviceConnect, "Pump")
    manualchar = GetPumpCharacteristics(service, "MODE")
    bolusChar = GetPumpCharacteristics(service, "BOLUS")

    while True:
        choice = raw_input("\n(1) MODE\n(2) BOLUS\n\tChoice?")

        if choice == "1":
            BLEWriter(manualchar)
        elif choice == "2":
            BLEWriter(bolusChar)

    DisconnectDevice(deviceConnect)


if __name__ == '__main__':
    sys.exit(main(sys.argv))
