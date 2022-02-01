use std::fmt;
use tungstenite::{connect, Message};

const URL: &str = "ws://livetiming.formula1.com/signalr";

enum Topic {
    Heartbeat,
    CarData,
    Position,
    ExtrapolatedClock,
    TopThree,
    RcmSeries,
    TimingStats,
    TimingAppData,
    WeatherData,
    TrackStatus,
    DriverList,
    RaceControlMessages,
    SessionInfo,
    SessionData,
    LapCount,
    TimingData,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signalr() {
        let (mut socket, response) = connect(URL).expect("Can't connect");

        println!("Connected to the server");
        println!("Response HTTP code: {}", response.status());
        println!("Response contains the following headers:");
        for (ref header, _value) in response.headers() {
            println!("* {}", header);
        }

        loop {
            let msg = socket.read_message().expect("Error reading message");
            println!("Received: {}", msg);
        }
    }
}
