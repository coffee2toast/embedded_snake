#!/bin/bash

openocd -d0 -f interface/stlink-v2-1.cfg -f target/stm32f7x.cfg -c "init;targets;halt;flash write_image erase $@;reset run; shutdown;"
