# 明日方舟【宝石铭刻】计算器

## 运行

### Windows

可以下载预制品.

### MacOS / Linux

需要 Rust 环境，安装方法见 https://www.rust-lang.org/zh-CN/tools/.

## 使用方式

```
Usage: iner-calc.exe <BOARD_SIZE> <NESRE> <PET> <GABE> <SHAY>

Arguments:
  <BOARD_SIZE>  解锁的操作台数量
  <NESRE>       待加工火焰伊纳数量
  <PET>         待加工天空伊纳数量
  <GABE>        待加工草伊纳数量
  <SHAY>        待加工沙伊纳数量

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### 编译运行

## 命名

![各类伊纳对应的名称](/readmefiles/image.png)

## 运行方式

运行 `cargo run --release`. **不使用 `--release` 大约会带来 10x 的额外时间开销。因此请务必使用 `cargo run --release` 运行。**

本计算器使用 [Rayon](https://github.com/rayon-rs/rayon) 进行并行化计算，计算过程**将占用您所有的CPU核心，对于CPU运算能力较弱的电脑来说，运行时或许会造成电脑卡顿或风扇高速运转，属于正常现象**。

### 运行结果解读

```
[000.00%]: 0000000 of 3628800, 0.00k iter/s
[012.76%]: 0462885 of 3628800, 3240.20k iter/s
[025.12%]: 0911438 of 3628800, 3139.87k iter/s
[036.22%]: 1314258 of 3628800, 2819.74k iter/s
[048.33%]: 1753824 of 3628800, 3076.96k iter/s
[060.25%]: 2186405 of 3628800, 3028.07k iter/s
[074.88%]: 2717402 of 3628800, 3716.98k iter/s
[089.91%]: 3262560 of 3628800, 3816.11k iter/s
[100.00%]: 3628800 of 3628800, 2563.68k iter/s
======== CALCULATION FINISHED ========
Score: 176485
Board: [Some(FilterI), Some(MingleII), Some(CrystalI), Some(CrystalII), Some(CrystalIII), Some(MingleI)]
Technique: [ExtractI, ExtractII, ExtractIII, ExtractIV, FilterII, FilterIII, MingleIII]
```

- `Score`: 本次能够获得的最高分数；
- `Board`: 操作台上从左到右依次应当放置的工艺，`Some(X)` 表示应该放置，`None`表示应该空置；
- `Technique`：剩余在场下的工艺。
