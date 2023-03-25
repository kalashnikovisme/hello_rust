MAX=15000000 # 50000 ~= 1 MB
echo "{"
echo $(sed '1d;$d' test.json)         # ChatGPT
echo ","
i=1
while [ $i -le $MAX ]; do
  array=(1 2 3 4 5)
  values=(null true 5 "\"something\"" $array "{ \"key\": \"value\" }")
  random_index=$((RANDOM % ${#values[@]}))        # ChatGPT
  random_element=${values[$random_index]}         # ChatGPT
  echo -ne "\"key$i\": $random_element"
  if [ "$i" -ne "$MAX" ]; then
    echo ","
  fi
  let i=i+1
done
echo "}"

