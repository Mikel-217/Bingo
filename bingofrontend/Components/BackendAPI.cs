using Data;

namespace ApiBack;


public class RustAPI {
    
    private readonly HttpClient _httpClient;
    private const string URL = "https://bingo.mastermc.de/api/data";

    public RustAPI(HttpClient httpClient) {
        _httpClient = httpClient;   
    }

    public async Task<string> GetAsync() {
        var responce = await _httpClient.GetFromJsonAsync<BingoWord>(URL);
        return responce?.recevedData!;
    }

    public async Task SendData(string Data) {
        
    }
}
