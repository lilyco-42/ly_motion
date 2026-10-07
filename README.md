# ly_motion
关于图片素材运动,特效UI界面,人物贴图高效组织,高性能跨平台渲�?复杂人物动作�?
一些我比较喜欢的交互动�?
![spine_明日方舟动画](https://space.bilibili.com/175547370/dynamic)
![Unkown_Alice in Cradle ](https://aicwiki.com/)
![apk_艾希]('"C:\Users\liuqi\Downloads\安卓手机游戏《艾希Ⅴ1.1.1》[mod版].apk"')
![motion_pcl]("https://github.com/Meloong-Git/PCL")
# 那么开始规划项�?
```call lyco-skill:OODA
观察: 观看动画这个B站博主视�?ta基于明日方舟小人spine素材,高清�?spine做出某种动作,持枪,设计.
存在图片结构处理,骨骼绑定�?.
分析: 所谓动�?就是一团图片变换和移动形成时间上的流畅变化效果,达成某种引导目的. 
最核心不可缺少�?图片,时间,结构,前后叠加�?
决策: 先实现渲染一个图�?比如PV开�?红色方块移动的gif.我们已经选定了rust编程语言
行动:寻找确定的库.
```
```call 网页搜索:google:image rust 
阅读 https://docs.rs/image/latest/image/ 记为 image_crate
阅读 https://crates.io/crates/image
```
```动作 总结:image_crate
[这个 crate 提供了图像编码和解码�?Rust 原生实现，以及一些基本的图像处理函数。更多文档目前也可以�?README.md 文件中找到，该文件在 GitHub 上查看最为便捷�?

该库旨在解决两个核心问题：统一的图像编码接口和用于存储图像内容的简单通用缓冲区。用户可以单独使用其中任何一个功能。该库专注于提供一套精简且稳定的常用操作，并可通过其他专用库进行补充。此外，该库也倾向于采用依赖项少且安全的方案�?
] => [一个图片处理库]

```
``` call 网页查看:https://github.com/image-rs/image
Readme 都是很不错的示例
```
```动作
动作 复制:readme.md
动作 新建文件�? image
```
