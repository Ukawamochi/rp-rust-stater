# Linux環境でUSB周りの権限を許可する方法
## 1,Raspberry Pi Pico のベンダー/プロダクトIDを調べる
まずは `lsusb` コマンドで接続された USB デバイス一覧を確認します:
```bash
lsusb
```
出力例:
```
Bus 001 Device 005: ID 2e8a:0003 Raspberry Pi Trading Ltd
```
ここで `ID 2e8a:0003` の部分がベンダーID:プロダクトID です。

## 2,udev ルールの作成
`/etc/udev/rules.d/99-rp2040.rules` を作成し、以下を記述します:
```bash
sudo tee /etc/udev/rules.d/99-rp2040.rules <<EOF
SUBSYSTEM=="usb", ATTRS{idVendor}=="2e8a", ATTRS{idProduct}=="0003", MODE="0666", GROUP="plugdev"
EOF
```

## 3,ルールの適用
```bash
sudo udevadm control --reload-rules
sudo udevadm trigger
```
USB を抜き差ししてください。