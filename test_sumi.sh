#!/bin/bash

echo "🚀 Starting sumi process test..."
echo "------------------------------"

for i in {1..20}
do
    timestamp=$(date +"%T")
    
    if (( i % 3 == 0 )); then
        echo "[$timestamp] ❌ ERROR: System failure detected at step $i" >&2
        sleep 0.2
        echo "[$timestamp] ⚠️  RETRYING: Attempting to recover process..." >&2
    else
        echo "[$timestamp] ✅ INFO: Processing task number $i..."
        echo "[$timestamp] ⚙️  STATUS: Everything running smoothly."
    fi
    
    sleep 0.5
done

echo "------------------------------"
echo "✅ Test completed successfully."
