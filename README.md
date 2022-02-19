# 組み込みRust: Raspberry Pi Pico でLチカ (SysTickを利用)

## 概要

Cortex-M のSysTickで周期的な割り込みを発生させて、SysTick例外ハンドラでRaspberry Pi PicoのLEDを点滅させます。

## ポイント

* 例外ハンドラからHALを操作する
  * `static` 変数で共有
  * 排他制御 `Mutex<RefCell<Option<共有変数>>>`
* SysTick
  * 周期割り込みの発生方法
  * 例外ハンドラの書き方

## 注意

SysTick は `cortex_m::delay::Delay` も必要としています。
ユーザプログラムが自前で SysTick を制御する場合は、 `cortex_m::delay::Delay` は使えません。

## ライセンス

MIT / Apache 2.0 のデュアルライセンスとします。

## 連絡先

KIKUCHI Yusuke
ac100v.net@gmail.com
