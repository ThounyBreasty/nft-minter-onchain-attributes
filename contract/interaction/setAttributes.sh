source devnet.snippets.sh

for i in {1..2401..100}
do
    at=@
    start="$(printf '%04x' $i)"
    zeroStart=${start:0:2}

    if [[ $zeroStart == "00" ]];then
        start=${start:2}
    else
        start=$start
    fi

    j=$(($i+100))
    end="$(printf '%04x' $j)"
    zeroEnd=${end:0:2}
    if [[ $zeroEnd == "00" ]];then
        end=${end:2}
    else
        end=$end
    fi

    ATTR_START=${ATTRIBUTES%%${at}${end}*}
    ATTR_END=${ATTR_START##*${at}${start}}
    ATTRIBUTES=setAttributes${at}${start}${ATTR_END}
    setAttributes
    echo $start
    echo i $i j $j
    sleep 6
done