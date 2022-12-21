cd jarm_online_gui && npm run serve > /dev/null & pid=$!
pytest
echo $pid
kill -- -$pid  # FIXME process no longer exist