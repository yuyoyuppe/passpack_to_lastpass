# Description
Simple utility to convert latest PassPack csv-exported data into a format best compatible with LastPass. It expects the following format from PassPack:
```csv
"Entry Name","URL","User ID","Password","Notes","Shared Notes","Tags","Extra Fields","Email"
```


# Usage
```sh
passpack_to_lastpass -i R:\passpack.csv -o R:\lastpass.csv
``` 