listener 1883

user root

persistence true
persistence_location $MOSQUITTO_DIR
autosave_interval 300

max_inflight_messages 2
max_queued_messages 525600

include_dir $INCLUDE_DIR