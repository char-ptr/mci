mkdir .\mappings
cd .\mappings
curl https://raw.githubusercontent.com/FabricMC/intermediary/master/mappings/1.19.tiny -o maps.tiny
git clone --depth 1 --sparse -b 1.19 https://github.com/FabricMC/yarn yarn-maps
cd ./yarn-maps
git sparse-checkout set mappings