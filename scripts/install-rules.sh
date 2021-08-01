#!/bin/bash

if sudo [ -d "/etc/udev/rules.d" ] 
then
    echo "Udev rules directory exists." 
    echo "Installing rules..."
    sudo cp ./rules/75-gigabyte-monitor.rules /etc/udev/rules.d/75-gigabyte-monitor.rules
    sudo chown root:root /etc/udev/rules.d/75-gigabyte-monitor.rules
    sudo chmod 644 /etc/udev/rules.d/75-gigabyte-monitor.rules
else
    echo "Udev rules directory /etc/udev/rules.d does not exist."
fi