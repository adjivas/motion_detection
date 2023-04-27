#!/bin/sh

export HTTP_CGI_HOST="http://{MY IP}:8080/cgi-bin/snap.cgi"
export HTTP_CGI_USER="{MY USERNAME}"
export HTTP_CGI_PASS="{MY PASSWORD}"
export MQTT_NAME="{MY SERVICE NAME}"
export MQTT_HOST="localhost"
export MQTT_PORT=1883
export MQTT_PUBLISH="home/doorbell/motion"
export MOTION_SENSIBILITY="0.90"
