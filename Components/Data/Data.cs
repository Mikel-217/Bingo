namespace Data;

public class Bingodata {
    public List<string>? bingoWords { get; set; } = new List<string>();
}

public class Bingorows {
    public int[] bingorow1 = {0, 5, 10, 15, 20};
    public int[] bingorow2 = {1, 6, 11, 16, 21};
    public int[] bingorow3 = {2, 7, 12, 17, 22};
    public int[] bingorow4 = {3, 8, 13, 18, 23};
    public int[] bingorow5 = {4, 9, 14, 19, 24};
}

public class NewBingo {
    public string? newWord { get; set; }
    public List<string> newWordslist { get; set; } = new List<string>();
}

public class Userinp {
    public string? inputpw { get; set; }
    public string? inputuser { get; set; }
}