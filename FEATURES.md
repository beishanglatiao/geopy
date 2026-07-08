# geopy GIS 工具箱 - 功能实现进度

最后更新：2026-07-05

---

## 概述

geopy 是一个高性能 GIS 工具箱，采用 Python 前端 + Rust 核心架构，基于 Whitebox crates 进行地理空间操作。

**状态图例：**
- ✅ 已完成
- 🚧 进行中
- 🔲 未开始
- ❌ 受阻

---

## 1. 统计工具 (`src/stats_tools/`)

| 功能 | 描述 | 优先级 | 状态 | 文件 |
|------|------|--------|------|------|
| 分区直方图 | 统计每个多边形内的整数像素值 | 🔴 高 | ✅ | [zonal_histogram.rs](src/stats_tools/zonal_histogram.rs) |
| 分区统计 | 计算每个多边形的均值、总和、标准差、最小值、最大值 | 🔴 高 | 🔲 | zonal_statistics.rs |
| 栅格统计 | 全局统计：均值、标准差、众数、分位数 | 🔴 高 | 🔲 | raster_statistics.rs |
| 面积制表 | 按栅格值统计多边形内各值的面积 | 🟡 中 | 🔲 | tabulate_area.rs |

---

## 2. 分析工具 (`src/analysis.rs`)

| 功能 | 描述 | 优先级 | 状态 | 文件 |
|------|------|--------|------|------|
| 坡度计算 | 从 DEM 计算坡度 | 🔴 高 | 🔲 | analysis.rs |
| 坡向计算 | 从 DEM 计算坡向 | 🔴 高 | 🔲 | analysis.rs |
| 山体阴影 | 从 DEM 生成山体阴影 | 🔴 高 | 🔲 | analysis.rs |
| 流量累积 | 流量累积分析 | 🟡 中 | 🔲 | analysis.rs |
| 曲率计算 | 平面曲率和剖面曲率 | 🟡 中 | 🔲 | analysis.rs |

---

## 3. 数据管理 (`src/management.rs`)

| 功能 | 描述 | 优先级 | 状态 | 文件 |
|------|------|--------|------|------|
| 矢量投影转换 | 将矢量转换到不同坐标系 | 🔴 高 | 🔲 | management.rs |
| 栅格投影转换 | 将栅格转换到不同坐标系 | 🔴 高 | 🔲 | management.rs |
| 栅格裁剪 | 按矢量范围裁剪栅格 | 🔴 高 | 🔲 | management.rs |
| 矢量裁剪 | 用另一个矢量裁剪矢量 | 🟡 中 | 🔲 | management.rs |
| 栅格格式转换 | 格式转换（GeoTIFF ↔ 其他格式） | 🟡 中 | 🔲 | management.rs |
| 矢量格式转换 | 格式转换（Shapefile ↔ GPKG ↔ GeoJSON） | 🟡 中 | 🔲 | management.rs |
| 栅格重采样 | 栅格重采样 | 🟡 中 | 🔲 | management.rs |

---

## 4. 空间分析 (`src/sa.rs`)

| 功能 | 描述 | 优先级 | 状态 | 文件 |
|------|------|--------|------|------|
| 缓冲区分析 | 缓冲区分析 | 🔴 高 | 🔲 | sa.rs |
| 交集分析 | 交集分析 | 🔴 高 | 🔲 | sa.rs |
| 并集分析 | 并集分析 | 🔴 高 | 🔲 | sa.rs |
| 空间连接 | 空间连接操作 | 🔴 高 | 🔲 | sa.rs |
| 融合分析 | 按属性合并要素 | 🟡 中 | 🔲 | sa.rs |
| 擦除分析 | 擦除要素的部分区域 | 🟡 中 | 🔲 | sa.rs |
| 对称差分析 | 对称差分析 | 🟢 低 | 🔲 | sa.rs |

---

## 5. IO 封装增强 (`src/io/wrapper.rs`)

| 功能 | 描述 | 优先级 | 状态 | 文件 |
|------|------|--------|------|------|
| Raster.read() | 从文件读取栅格 | 🔴 高 | ✅ | [wrapper.rs](src/io/wrapper.rs) |
| Raster.write() | 将栅格写入文件 | 🔴 高 | ✅ | [wrapper.rs](src/io/wrapper.rs) |
| Raster.get()/set() | 像素读写 | 🔴 高 | ✅ | [wrapper.rs](src/io/wrapper.rs) |
| Raster.x_min/x_max/y_min/y_max | 边界范围 | 🟡 中 | ✅ | [wrapper.rs](src/io/wrapper.rs) |
| Raster.cell_size_x/y | 像元大小 | 🟡 中 | ✅ | [wrapper.rs](src/io/wrapper.rs) |
| Raster.data_type | 数据类型 | 🟡 中 | ✅ | [wrapper.rs](src/io/wrapper.rs) |
| Raster.crs | 获取坐标系信息 | 🟡 中 | ✅ | [wrapper.rs](src/io/wrapper.rs) |
| Layer.read() | 从文件读取矢量 | 🔴 高 | ✅ | [wrapper.rs](src/io/wrapper.rs) |
| Layer.write() | 将矢量写入文件 | 🔴 高 | ✅ | [wrapper.rs](src/io/wrapper.rs) |
| Layer.crs | 获取坐标系信息 | 🟡 中 | ✅ | [wrapper.rs](src/io/wrapper.rs) |
| Layer.geom_type | 几何类型 | 🟡 中 | ✅ | [wrapper.rs](src/io/wrapper.rs) |

---

## 6. Python API (`geopy/`)

| 功能 | 描述 | 状态 | 文件 |
|------|------|------|------|
| zonal_histogram | 分区直方图的 Python 封装 | ✅ | [stats_tools.py](geopy/stats_tools.py) |
| Polars DataFrame 返回 | 将结果作为 Polars DataFrame 返回 | ✅ | [stats_tools.py](geopy/stats_tools.py) |
| 可选 output_path | 为 None 时跳过 Excel 写入 | ✅ | [stats_tools.py](geopy/stats_tools.py) |
| 文件路径自动递增 | 处理重复输出文件 | ✅ | [stats_tools.py](geopy/stats_tools.py) |

---

## 实现进度汇总

| 分类 | 总数 | 已完成 | 进行中 | 未开始 |
|------|------|--------|--------|--------|
| 统计工具 | 4 | 1 | 0 | 3 |
| 分析工具 | 5 | 0 | 0 | 5 |
| 数据管理 | 7 | 0 | 0 | 7 |
| 空间分析 | 7 | 0 | 0 | 7 |
| IO 封装 | 11 | 10 | 0 | 1 |
| Python API | 4 | 4 | 0 | 0 |
| **总计** | **38** | **15** | **0** | **23** |

**完成率：** 39.47% (15/38)

---

## 下一步计划

1. **高优先级**：分区统计、缓冲区、栅格裁剪、投影转换
2. **中优先级**：栅格统计、面积制表、山体阴影、坡度计算
3. **低优先级**：剩余空间分析工具
