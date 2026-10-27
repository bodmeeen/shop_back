#!/bin/bash

trap 'echo -e "\n Зупинено"; kill $(jobs -p); exit' SIGINT SIGTERM

cd "/home/max/Projects/site_rust_back/shop_back" || exit
cargo run --bin shop_back &
echo "Бек запущений"

cd "/home/max/Projects/site_ts_front" || exit
npm run dev &
echo "Фронт запущений"

wait
