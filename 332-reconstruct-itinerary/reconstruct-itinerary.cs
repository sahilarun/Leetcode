public class Solution {
    private Dictionary<string, PriorityQueue<string, string>> graph = new();

    public IList<string> FindItinerary(IList<IList<string>> tickets) {
        foreach (var ticket in tickets) {
            string from = ticket[0], to = ticket[1];
            if (!graph.ContainsKey(from))
                graph[from] = new PriorityQueue<string, string>();
            graph[from].Enqueue(to, to);
        }

        var route = new LinkedList<string>();
        DFS("JFK", route);
        return route.ToList();
    }

    private void DFS(string airport, LinkedList<string> route) {
        if (graph.ContainsKey(airport)) {
            var pq = graph[airport];
            while (pq.Count > 0) {
                string next = pq.Dequeue();
                DFS(next, route);
            }
        }
        route.AddFirst(airport);
    }
}
