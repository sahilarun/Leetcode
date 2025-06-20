defmodule Solution do
  def find_words(board, words) do
    trie = build_trie(words)
    board = Enum.map(board, &Enum.map(&1, fn ch -> <<ch>> end))
    rows = length(board)
    cols = length(hd(board))

    {found, _} =
      for i <- 0..(rows - 1), j <- 0..(cols - 1), reduce: {MapSet.new(), trie} do
        {acc_set, acc_trie} ->
          ch = board |> Enum.at(i) |> Enum.at(j)
          case Map.get(acc_trie, ch) do
            nil -> {acc_set, acc_trie}
            _ -> dfs(board, i, j, acc_trie, [], %{}, acc_set)
          end
      end

    Enum.filter(words, &MapSet.member?(found, &1))
  end

  defp dfs(board, i, j, trie, path, visited, found) do
    if i < 0 or j < 0 or i >= length(board) or j >= length(hd(board)) or Map.has_key?(visited, {i, j}) do
      {found, trie}
    else
      ch = board |> Enum.at(i) |> Enum.at(j)
      next = Map.get(trie, ch)

      if next == nil do
        {found, trie}
      else
        path = [ch | path]
        visited = Map.put(visited, {i, j}, true)
        word = Enum.reverse(path) |> Enum.join()

        found =
          if Map.has_key?(next, :end),
            do: MapSet.put(found, word),
            else: found

        {found, updated_next} =
          Enum.reduce([{1,0},{-1,0},{0,1},{0,-1}], {found, next}, fn {di, dj}, {facc, tacc} ->
            dfs(board, i + di, j + dj, tacc, path, visited, facc)
          end)

        # prune if empty
        updated_next = Map.delete(updated_next, :end)
        updated_trie =
          if map_size(updated_next) == 0,
            do: Map.delete(trie, ch),
            else: Map.put(trie, ch, updated_next)

        {found, updated_trie}
      end
    end
  end

  defp build_trie(words) do
    Enum.reduce(words, %{}, fn word, acc -> insert(String.graphemes(word), acc) end)
  end

  defp insert([h | t], trie) do
    Map.update(trie, h, insert(t, %{}), fn child -> insert(t, child) end)
  end

  defp insert([], trie), do: Map.put(trie, :end, true)
end
