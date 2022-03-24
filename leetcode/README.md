LeetCode
========

### LeetCode Algorithm

(Notes: "&hearts;" means you need to buy a book from Leetcode)


| #   | Title | Solution                         | Difficulty | Tag        | Note        |
|-----| ----- |----------------------------------|------------|------------|-------------|
| 3   |[Longest Substring Without Repeating Characters](https://leetcode.com/problems/longest-substring-without-repeating-characters/) | [Rust](./src/longest_substring_without_repeating_characters.rs) | Medium | String          | HashMap |
| 2   |[Add Two Numbers](https://leetcode.com/problems/add-two-numbers/) | [Rust](./src/add_two_numbers.rs) | Medium | LinkedList | Recursion |
| 1   |[Two Sum](https://leetcode.com/problems/two-sum/) | [Rust](./src/two_sum.rs)         | Easy       | Array      | Vec,HashMap |

# 数据结构



# 常用算法

贪婪算法、动态规划、分治算法、分治法、分支限界算法

### 贪婪算法

贪婪算法可以获取到问题的局部最优解，不一定能获取到全局最优解，同时获取最优解的好坏要看贪婪策略的选择。特点就是简单，能获取到局部最优解。
就像打狗棍法，同一套棍法，洪七公和鲁有脚的水平就差太多了，因此同样是贪婪算法，不同的贪婪策略会导致得到差异非常大的结果。

#### 步骤
1. 将问题分解为多个子问题
2. 得到问题的局部最优解
3. 合并子问题的局部最优解

http://blog.csdn.net/changyuanchn/article/details/51417211

### 动态规划

当最优化问题具有重复子问题和最优子结构的时候，就是动态规划出场的时候了。动态规划算法的核心就是提供了一个memory来缓存重复子问题的结果，
避免了递归的过程中的大量的重复计算。动态规划算法的难点在于怎么将问题转化为能够利用动态规划算法来解决。
当重复子问题的数目比较小时，动态规划的效果也会很差。如果问题存在大量的重复子问题的话，那么动态规划对于效率的提高是非常恐怖的。
就像斗转星移武功，对手强它也会比较强，对手若，他也会比较弱。

#### 适用情况
* 最优化原理：问题的最优解所包含的子问题的解也是最优的，即最优子结构
* 无后效性：某个状态一旦确定，就不受以后决策的影响
* 有重叠子问题

递推关系是从次小的问题开始到较大问题的转化，往往可以用递归来实现，可以利用之前产生的子问题的解来减少重复的计算


http://blog.csdn.net/changyuanchn/article/details/51420028
http://blog.csdn.net/changyuanchn/article/details/51429979

### 分治算法

分治算法的逻辑更简单了，就是一个词，分而治之。分治算法就是把一个大的问题分为若干个子问题，然后在子问题继续向下分，一直到base cases，
通过base cases的解决，一步步向上，最终解决最初的大问题。分治算法是递归的典型应用。

#### 适用情况
* 问题分解为小问题后容易解决
* 问题可以分解为小问题，即最优子结构
* 分解后的小问题解可以合并为原问题的解
* 小问题之间互相独立

实例：二分查找、快速排序、合并排序、大整数乘法、循环赛日程表

http://blog.csdn.net/changyuanchn/article/details/17150109
http://blog.csdn.net/changyuanchn/article/details/51465175

### 回溯算法

回溯算法是深度优先策略的典型应用，回溯算法就是沿着一条路向下走，如果此路不同了，则回溯到上一个
分岔路，在选一条路走，一直这样递归下去，直到遍历万所有的路径。八皇后问题是回溯算法的一个经典问题，还有一个经典的应用场景就是迷宫问题。

#### 步骤
1. 确定解空间
2. 确定节点的扩展搜索规则
3. 深度优先方式搜索解空间，用剪枝法避免无效搜索

http://blog.csdn.net/changyuanchn/article/details/17354461

### 分支限界算法

回溯算法是深度优先，那么分支限界法就是广度优先的一个经典的例子。回溯法一般来说是遍历整个解空间，获取问题的所有解，
而分支限界法则是获取一个解（一般来说要获取最优解）。

* 与回溯法类似，也是在解空间里搜索解得算法，不同点是，回溯法寻找所有解，分支界限法搜索一个解或者最优解
* 分支：广度优先策略或者最小耗费（最大效益）优先
* 分支搜索方式：FIFO、LIFO、优先队列式、分支界限搜索算法

http://blog.csdn.net/changyuanchn/article/details/17102037

# 学习参考
* https://github.com/changgyhub/leetcode_101/
* https://github.com/haoel/leetcode/tree/master/algorithms/cpp
* https://github.com/pezy/LeetCode
* https://github.com/labuladong/fucking-algorithm
