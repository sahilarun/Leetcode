class Codec {
    fun serialize(root: TreeNode?): String {
        val sb = StringBuilder()
        fun dfs(node: TreeNode?) {
            if (node == null) {
                sb.append("#,")
                return
            }
            sb.append("${node.`val`},")
            dfs(node.left)
            dfs(node.right)
        }
        dfs(root)
        return sb.toString()
    }

    fun deserialize(data: String): TreeNode? {
        val nodes = data.split(",").iterator()
        fun dfs(): TreeNode? {
            if (!nodes.hasNext()) return null
            val v = nodes.next()
            if (v == "#") return null
            val node = TreeNode(v.toInt())
            node.left = dfs()
            node.right = dfs()
            return node
        }
        return dfs()
    }
}