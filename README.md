# 明日方舟【宝石铭刻】计算器

## 安装与运行

需要 Rust 环境，安装方法见 https://www.rust-lang.org/zh-CN/tools/.

在有 Rust 环境的电脑中：

```
cargo install iner-calc
```

即可安装。

## 使用方式

这是命令行工具。使用方式见下面的描述：

```
Usage: iner-calc <BOARD_SIZE> <NESRE> <PET> <GABE> <SHAY>

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


## 命名

![各类伊纳对应的名称](/readmefiles/image.png)


## 运行方式

本计算器使用 [Rayon](https://github.com/rayon-rs/rayon) 进行并行化计算，计算过程**将占用您所有的CPU核心，对于CPU运算能力较弱的电脑来说，运行时或许会造成电脑卡顿或风扇高速运转，属于正常现象，不会损坏您的电脑:D**

### 运行结果解读

下面是一个示例数据：

```
>>> iner-calc 6 10 70 10 10
[000.00%]: 0000000 of 3628800, 0.00k iter/s
[014.38%]: 0521927 of 3628800, 3653.49k iter/s
[028.75%]: 1043234 of 3628800, 3649.15k iter/s
[043.90%]: 1592869 of 3628800, 3847.45k iter/s
[058.30%]: 2115732 of 3628800, 3660.04k iter/s
[073.33%]: 2661023 of 3628800, 3817.04k iter/s
[088.36%]: 3206576 of 3628800, 3818.87k iter/s
[100.00%]: 3628800 of 3628800, 2955.57k iter/s
======== CALCULATION FINISHED ========
Score: 87015
Board: [Some(FilterI), Some(MingleII), Some(CrystalI), Some(CrystalII), Some(CrystalIII), Some(MingleI)]
Technique: [ExtractI, ExtractII, ExtractIII, ExtractIV, FilterII, FilterIII, MingleIII]
```

- `Score`: 本次能够获得的最高分数；
- `Board`: 操作台上从左到右依次应当放置的工艺，`Some(X)` 表示应该放置，`None`表示应该空置；
- `Technique`：剩余在场下的工艺。
- `ExtractN`: 萃雕工艺（火焰伊纳线，`N`代表`I/II/III/IV`）
- `FilterN`: 滤纯工艺（草伊纳线，`N`代表`I/II/III`）
- `MingleN`: 交糅工艺（天空伊纳线，`N`代表`I/II/III`）
- `CrystalN`: 落晶工艺（沙伊纳线，`N`代表`I/II/III`）
