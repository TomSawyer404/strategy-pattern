# Strategy 模式

策略模式指对象有某个行为，但是在不同的场景中，该行为有不同的实现算法。比如每个人都要“交个人所得税”，但是“在美国交个人所得税”和“在英国交个人所得税”就有不同的算税方法。

策略模式：

- 定义了一族算法（业务规则）；
- 封装了每个算法；
- 这族的算法可互换代替（interchangeable）

策略模式可以减少`switch`语句或条件语句的调用，用相同的方法调用就可达到不同的行为。

在 Rust 中，Strategy 模式就是`trait`的应用。

# 实战

一年一度的万圣节又到了，[Steam](https://store.steampowered.com/about/) 开启了节日特惠活动。这一次 G胖 对使用不同支付方式的玩家给出不同的优惠政策。

![shop](https://img2.baidu.com/it/u=513755520,4189436400&fm=26&fmt=auto)

G胖 生成了一组随机数，最后他选定，使用支付宝付款的玩家有`-50%`的优惠，而使用微信支付的玩家有`-66%`的优惠。现在我们用策略模式来重现这一过程。

# 参考资料

- [策略模式-Wikipedia](https://zh.wikipedia.org/zh-cn/%E7%AD%96%E7%95%A5%E6%A8%A1%E5%BC%8F)
- [lpxxn的示例代码](https://github.com/lpxxn/rust-design-pattern/blob/master/behavioral/strategy.rs)
- [yukihir0的示例代码](https://github.com/yukihir0/rust_design_pattern/blob/master/strategy/src/main.rs)
- [Strategy - Rust Design Patterns](https://rust-unofficial.github.io/patterns/patterns/behavioural/strategy.html)
- [Golang策略模式](https://www.topgoer.cn/docs/golang-design-pattern/Strategy)

---
