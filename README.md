# ly_motion
关于图片素材运动,特效UI界面,人物贴图高效组织,高性能跨平台渲染,复杂人物动作等
一些我比较喜欢的交互动作
![spine_明日方舟动画](https://space.bilibili.com/175547370/dynamic)
![Unkown_Alice in Cradle ](https://aicwiki.com/)
![apk_艾希]('"C:\Users\liuqi\Downloads\安卓手机游戏《艾希Ⅴ1.1.1》[mod版].apk"')
![motion_pcl]("https://github.com/Meloong-Git/PCL")
# 那么开始规划项目
```call lyco-skill:OODA
观察: 观看动画这个B站博主视频,ta基于明日方舟小人spine素材,高清化,spine做出某种动作,持枪,设计.
存在图片结构处理,骨骼绑定等..
分析: 所谓动画,就是一团图片变换和移动形成时间上的流畅变化效果,达成某种引导目的. 
最核心不可缺少的.图片,时间,结构,前后叠加层
决策: 先实现渲染一个图片,比如PV开场,红色方块移动的gif.我们已经选定了rust编程语言
行动:寻找确定的库.
```
```call 网页搜索:google:image rust 
阅读 https://docs.rs/image/latest/image/
阅读 https://crates.io/crates/image
```
