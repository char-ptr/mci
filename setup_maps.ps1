param($version)
echo "downloading mappings for $version"
if (Test-Path .\mappings ) {
    echo "removing old mappings"
    rm -Recurse -Force .\mappings
}

mkdir .\mappings
curl https://raw.githubusercontent.com/FabricMC/intermediary/master/mappings/$version.tiny -o ./mappings/maps.tiny
git clone --depth 1 --sparse -b $version https://github.com/FabricMC/yarn ./mappings/yarn-maps
git -C ./mappings/yarn-maps sparse-checkout set mappings
