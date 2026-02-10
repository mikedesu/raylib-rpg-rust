#!/usr/bin/zsh 
a=$(scc -i rs)
loc=$(echo $a | grep -i rust | awk '{print $3}')
estimated_cost=$(echo $a | grep -i cost | awk '{print $6}' | sed 's/,//g')
estimated_people=$(echo $a | grep -i people | awk '{print $5}')
estimated_schedule=$(echo $a | grep -i schedule | awk '{print $5 " " $6}')
timestamp=$(date +%s)
#echo $timestamp
#echo $loc lines of code
#echo $estimated_cost
#echo $estimated_people people
#echo $estimated_schedule
#echo "timestamp,loc,estimated_cost,estimated_people,estimated_schedule"
echo $timestamp,$loc,$estimated_cost,$estimated_people,$estimated_schedule
