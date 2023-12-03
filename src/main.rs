use std::collections::HashMap;

fn main() {
    day2();
}

fn day2() {
    let input = "Game 1: 1 green, 1 blue, 1 red; 3 green, 1 blue, 1 red; 4 green, 3 blue, 1 red; 4 green, 2 blue, 1 red; 3 blue, 3 green
Game 2: 9 blue, 7 red; 5 blue, 6 green, 1 red; 2 blue, 10 red, 9 green; 3 green, 14 red, 5 blue; 8 red, 3 blue, 4 green; 8 green, 14 blue, 10 red
Game 3: 11 green, 8 blue, 7 red; 3 green, 4 red, 9 blue; 3 red, 4 green, 8 blue; 11 green, 1 red, 16 blue
Game 4: 3 blue, 20 green, 2 red; 1 green, 3 red, 3 blue; 1 blue, 9 green; 4 red, 17 green; 12 green, 3 red
Game 5: 2 red, 1 blue, 4 green; 6 blue, 2 green; 2 red, 5 green
Game 6: 1 green, 7 red; 1 blue, 3 green, 1 red; 1 blue, 2 red, 2 green; 1 blue, 1 green, 2 red; 3 red; 8 red, 1 green, 1 blue
Game 7: 13 green, 5 blue, 1 red; 9 green, 6 blue; 11 green, 2 blue, 2 red
Game 8: 2 red, 11 green, 6 blue; 5 green, 3 blue; 3 blue, 3 green, 5 red; 5 blue, 9 green, 1 red
Game 9: 3 blue, 5 green, 8 red; 4 green, 19 blue, 4 red; 4 red, 17 blue
Game 10: 2 green, 8 red, 4 blue; 1 green, 5 red, 9 blue; 3 green, 2 red, 4 blue; 2 green, 5 red, 2 blue; 6 green, 4 blue; 10 blue, 8 green, 8 red
Game 11: 3 green, 1 blue, 9 red; 2 blue, 1 red, 9 green; 1 blue, 9 green, 7 red; 8 red, 6 green
Game 12: 5 green; 8 green, 7 red, 1 blue; 8 blue, 8 green, 14 red; 6 red, 14 green; 14 green, 3 red, 8 blue
Game 13: 7 red, 2 green, 4 blue; 5 red, 3 blue, 8 green; 10 green, 1 red, 4 blue; 7 green, 1 red, 13 blue; 11 green, 12 blue, 2 red
Game 14: 9 green, 4 red; 7 green, 4 blue, 5 red; 2 red, 2 green; 3 green, 2 red, 8 blue; 7 green, 6 red, 8 blue
Game 15: 19 blue, 1 green; 1 red, 5 blue; 3 green, 8 blue; 1 red, 13 blue, 3 green
Game 16: 8 red, 11 blue, 3 green; 14 green, 1 red, 12 blue; 6 green, 1 red, 6 blue; 1 red, 6 blue, 17 green; 2 green, 8 blue, 3 red
Game 17: 3 red, 1 blue; 1 blue, 2 red, 1 green; 1 red; 3 red, 2 green
Game 18: 8 green, 2 red; 1 blue, 6 red; 8 green, 7 red, 2 blue; 1 red; 6 green, 1 blue, 3 red
Game 19: 4 blue, 2 green; 4 green, 18 blue, 2 red; 14 green, 4 blue; 1 red, 3 blue, 18 green; 11 blue, 3 red; 14 green, 4 red, 6 blue
Game 20: 7 blue; 1 blue, 6 green, 1 red; 1 red, 3 blue, 10 green; 7 green, 4 blue, 1 red; 6 green, 6 blue, 1 red; 1 red, 5 blue, 17 green
Game 21: 11 blue, 9 red; 8 red, 2 blue; 2 red, 11 blue, 2 green
Game 22: 4 green, 9 blue, 2 red; 2 blue, 8 green; 2 green, 2 red, 6 blue
Game 23: 7 green, 7 blue; 6 blue, 11 green; 1 red, 14 green; 15 green, 4 blue, 1 red; 1 red, 5 blue, 3 green; 1 red, 1 blue, 13 green
Game 24: 5 green, 2 red, 2 blue; 1 blue, 3 green, 2 red; 2 blue, 7 green, 3 red
Game 25: 16 red, 8 green; 2 red, 3 blue; 10 green, 5 red, 4 blue; 9 red, 7 green; 7 red, 6 blue
Game 26: 3 red, 1 green; 5 red, 1 blue, 10 green; 8 red, 5 green
Game 27: 3 red, 2 blue; 6 red, 8 green; 5 green, 13 red, 2 blue; 4 red, 1 blue, 8 green; 14 red, 1 blue, 3 green; 2 red, 1 green, 2 blue
Game 28: 9 red, 10 blue; 9 red, 9 blue; 1 green, 6 red, 4 blue; 12 blue, 3 green, 2 red; 2 green, 12 red, 8 blue
Game 29: 4 red, 15 blue; 5 blue, 3 green, 6 red; 1 green, 9 blue, 1 red; 5 green, 8 red, 1 blue
Game 30: 4 green, 2 blue, 10 red; 1 red, 5 green, 6 blue; 15 red, 2 blue; 5 green, 10 red, 8 blue
Game 31: 6 green, 2 blue, 2 red; 5 green, 6 red, 6 blue; 3 blue, 5 red, 1 green; 4 green, 6 red, 9 blue; 4 red; 3 green, 1 red, 3 blue
Game 32: 8 green, 17 red, 17 blue; 11 red, 6 green, 13 blue; 14 red, 1 green, 1 blue; 1 green, 17 red, 4 blue; 5 green, 14 red, 15 blue; 15 blue, 8 green
Game 33: 2 red, 14 blue; 3 blue, 17 red, 4 green; 9 blue, 8 red; 5 red, 2 blue; 4 green, 16 red, 5 blue; 15 blue, 6 green, 17 red
Game 34: 12 green, 2 red, 1 blue; 3 blue, 9 red, 13 green; 2 blue, 17 green, 6 red; 6 green, 4 red, 3 blue; 2 red, 1 blue; 7 green, 3 blue, 7 red
Game 35: 4 blue, 5 red, 5 green; 6 green, 12 red, 6 blue; 3 green, 18 red, 2 blue; 13 red, 6 green, 9 blue; 3 green, 10 blue, 17 red; 1 green, 3 blue, 16 red
Game 36: 4 green, 3 blue, 1 red; 3 green, 3 red, 10 blue; 1 red, 8 green, 8 blue; 3 blue, 1 red; 2 red, 2 blue, 14 green
Game 37: 16 blue, 1 green; 9 blue; 7 red, 13 blue
Game 38: 6 green, 8 red, 3 blue; 5 blue, 4 green, 6 red; 5 blue, 4 green; 5 red, 3 green, 1 blue; 6 green, 4 blue, 15 red
Game 39: 10 blue, 4 green; 1 blue, 7 green, 5 red; 8 red, 2 blue
Game 40: 2 blue, 2 green, 6 red; 8 green, 4 red, 2 blue; 4 blue, 12 green, 6 red
Game 41: 4 green, 2 blue, 11 red; 6 red, 11 green; 4 blue, 2 red, 19 green; 3 green, 2 blue, 1 red
Game 42: 2 blue, 2 green, 4 red; 1 red, 4 blue, 8 green; 5 red, 2 blue, 15 green; 10 green, 5 red, 1 blue; 10 green, 1 blue; 2 blue
Game 43: 10 red, 19 green, 11 blue; 11 green, 1 red, 2 blue; 13 red, 6 blue, 3 green; 12 red, 10 green; 5 red, 19 green, 8 blue; 2 red, 10 green, 3 blue
Game 44: 7 blue, 8 red; 1 green, 12 red; 19 red, 3 green, 5 blue
Game 45: 12 red; 2 green, 5 red, 3 blue; 10 green, 2 blue, 4 red; 10 green, 7 red
Game 46: 4 blue, 4 red, 2 green; 7 green, 6 blue; 6 blue, 1 red, 4 green
Game 47: 4 green, 8 red, 1 blue; 4 green, 1 blue, 11 red; 14 red, 3 blue, 10 green; 15 green, 2 blue, 7 red
Game 48: 6 blue, 3 green, 1 red; 15 blue, 11 red, 3 green; 17 blue, 14 red; 2 green, 13 red; 5 red, 2 green, 4 blue
Game 49: 7 blue, 1 green; 8 red, 2 blue, 1 green; 1 red, 1 green, 2 blue; 3 red, 2 blue, 1 green
Game 50: 13 red, 2 green, 2 blue; 13 red, 6 green, 1 blue; 12 red, 8 green; 1 red, 3 green; 13 red; 2 blue, 11 red, 2 green
Game 51: 7 green, 4 blue, 2 red; 3 red, 7 green, 5 blue; 10 green, 2 blue; 14 green, 3 red, 4 blue; 2 blue, 2 red, 10 green
Game 52: 7 blue, 10 red; 7 red, 4 blue, 8 green; 12 red, 4 blue, 7 green; 7 green, 7 red; 17 green, 11 blue, 6 red; 8 green, 8 red, 18 blue
Game 53: 6 green, 2 red; 10 red, 13 green; 2 blue, 3 green, 5 red; 4 red, 4 green; 8 green, 1 red; 13 green, 2 blue, 10 red
Game 54: 5 red, 9 green, 5 blue; 6 red, 15 green, 10 blue; 8 blue, 3 green, 1 red; 12 blue, 3 red, 13 green
Game 55: 10 green, 4 red, 2 blue; 2 green, 1 red, 2 blue; 2 blue, 4 red, 8 green; 5 blue, 3 green
Game 56: 7 green, 9 red, 2 blue; 4 red, 1 blue, 3 green; 3 red, 4 blue, 6 green; 7 green, 2 red; 5 blue, 2 red, 3 green; 7 green, 7 red, 5 blue
Game 57: 11 blue, 5 green, 6 red; 18 red, 15 green, 10 blue; 5 green, 14 red, 6 blue; 1 green, 11 red, 7 blue; 11 blue, 7 red, 12 green
Game 58: 9 red, 6 blue, 6 green; 6 blue, 12 red, 3 green; 8 red, 1 blue, 20 green; 3 green, 3 red, 15 blue; 4 blue, 15 green, 3 red
Game 59: 18 red, 4 blue, 7 green; 11 blue, 19 red, 7 green; 10 red, 9 blue, 1 green; 8 red, 12 green, 4 blue; 6 green, 5 blue, 12 red; 2 blue, 14 green, 2 red
Game 60: 1 blue, 9 green, 6 red; 1 red, 1 blue, 13 green; 15 green, 4 red; 1 blue, 2 red, 15 green
Game 61: 9 green, 3 red, 2 blue; 1 green, 5 blue, 10 red; 17 red, 9 green, 5 blue; 10 red, 5 green, 5 blue
Game 62: 4 red, 2 green; 2 red; 5 red, 2 green, 2 blue; 3 green, 1 blue; 2 blue, 3 red, 3 green
Game 63: 4 red, 6 blue, 2 green; 3 green, 1 red, 5 blue; 7 blue, 5 green
Game 64: 8 blue, 12 red; 1 green, 6 red, 14 blue; 12 red, 13 blue
Game 65: 2 red, 8 green; 1 blue, 2 red, 5 green; 2 blue, 3 green; 1 green, 1 red
Game 66: 6 red, 8 blue, 2 green; 6 blue, 7 green; 7 green, 11 blue; 5 green, 4 red, 10 blue; 5 blue, 4 green; 6 blue, 6 green, 5 red
Game 67: 12 green, 4 red; 2 blue, 11 green, 6 red; 9 red, 2 green, 6 blue; 2 red, 8 blue, 18 green; 17 green, 7 blue, 6 red; 12 blue, 9 green
Game 68: 3 red, 9 blue, 1 green; 3 green, 11 blue; 12 blue, 9 red; 6 red, 13 green, 8 blue
Game 69: 3 red, 8 green, 3 blue; 6 green, 3 red; 11 green, 3 blue; 4 red, 3 green; 7 green, 4 blue, 6 red; 1 red, 2 blue
Game 70: 6 green, 4 blue; 7 red, 9 green, 14 blue; 12 red; 9 green, 10 red; 6 green, 16 blue, 8 red
Game 71: 4 blue, 6 red, 9 green; 6 green, 2 red; 8 green, 4 blue, 2 red; 1 red, 3 blue, 8 green; 9 green, 3 red, 3 blue; 4 red, 6 green
Game 72: 3 red, 3 green, 3 blue; 4 red, 1 green, 3 blue; 2 red, 2 green, 1 blue
Game 73: 7 green, 6 red, 7 blue; 2 green, 4 blue; 2 blue, 15 green, 4 red; 1 blue, 4 green, 2 red; 7 blue, 14 green
Game 74: 4 green, 2 red, 2 blue; 9 blue, 13 green, 4 red; 10 green, 12 blue, 7 red; 4 blue, 8 green, 7 red
Game 75: 3 red, 3 green; 3 green, 12 red; 18 red, 2 blue; 3 green, 9 red, 1 blue; 14 red, 1 green; 15 red
Game 76: 1 blue, 7 red, 8 green; 3 blue, 4 red, 1 green; 6 green, 6 red
Game 77: 2 green, 8 red; 5 green, 11 red, 1 blue; 5 red, 2 blue, 2 green; 6 red, 5 green, 2 blue; 11 red, 2 green, 1 blue; 2 red, 8 green, 2 blue
Game 78: 1 blue, 4 red, 10 green; 13 green, 4 red, 9 blue; 12 green, 6 blue, 3 red; 5 blue, 8 green, 6 red
Game 79: 9 red, 1 blue, 17 green; 9 green, 6 red; 15 green, 1 blue, 9 red; 1 blue, 8 red, 12 green; 11 green, 1 blue, 1 red
Game 80: 3 red, 3 blue, 1 green; 1 blue, 8 green; 10 green, 16 blue
Game 81: 15 blue, 2 red; 1 red, 8 blue; 2 green, 7 red, 11 blue; 19 blue, 8 red, 2 green; 20 red, 19 blue
Game 82: 3 red, 17 blue, 9 green; 10 red, 2 green, 17 blue; 13 red, 3 blue, 10 green; 11 red, 10 green, 18 blue; 1 green, 12 blue, 9 red; 3 red, 10 blue, 8 green
Game 83: 4 green, 2 blue, 14 red; 7 red, 2 blue, 7 green; 16 red, 7 green; 16 red, 2 green; 3 blue, 4 green, 15 red; 6 red, 1 blue, 4 green
Game 84: 4 blue, 1 green, 2 red; 7 blue, 6 red; 1 blue, 4 red, 3 green
Game 85: 5 blue, 1 red, 4 green; 14 blue, 7 green; 9 blue, 1 red, 7 green; 15 blue, 9 green; 8 blue, 6 green, 1 red
Game 86: 12 red, 12 blue, 7 green; 16 red, 11 green, 4 blue; 14 blue, 8 green, 8 red; 2 blue, 15 red, 6 green; 1 green, 6 red, 5 blue; 11 blue, 9 green
Game 87: 4 blue, 2 green, 6 red; 8 red, 3 green, 5 blue; 10 red, 1 green; 1 green, 3 blue, 1 red
Game 88: 2 green, 2 red, 4 blue; 4 blue, 4 green, 12 red; 2 green, 3 blue, 4 red; 2 green, 2 blue, 12 red; 4 blue, 8 red, 2 green
Game 89: 13 blue, 1 red, 5 green; 8 green, 16 blue, 5 red; 12 green, 2 red, 18 blue
Game 90: 7 red, 11 blue, 1 green; 8 green, 13 blue; 7 green, 16 blue; 5 green, 13 red, 11 blue; 5 blue, 10 green, 3 red
Game 91: 1 blue; 1 blue, 3 green; 1 green, 2 red
Game 92: 16 red, 4 blue, 14 green; 15 red, 7 blue, 13 green; 7 green, 13 red, 8 blue; 4 blue, 9 red, 5 green; 6 red, 7 blue, 8 green; 14 green, 7 red, 10 blue
Game 93: 2 red, 6 blue, 7 green; 8 green, 3 red, 10 blue; 1 green, 4 red; 2 red, 2 green; 8 blue, 7 green
Game 94: 2 green, 3 blue, 5 red; 10 blue; 1 green, 7 red; 3 blue, 1 green, 12 red
Game 95: 13 blue, 5 red; 9 blue, 3 red, 7 green; 10 green, 4 red, 12 blue; 14 blue; 7 green, 2 blue, 1 red
Game 96: 5 red, 2 blue; 4 red; 1 green, 2 blue
Game 97: 9 red, 10 green, 3 blue; 2 green, 15 red, 1 blue; 2 blue, 16 green, 16 red; 8 green, 14 red; 16 red
Game 98: 18 green, 16 red, 1 blue; 3 red, 2 blue, 20 green; 1 blue, 20 green, 14 red; 14 red, 2 green
Game 99: 7 red, 9 green, 5 blue; 6 blue, 1 green; 4 green, 5 blue, 1 red; 8 green, 6 red, 7 blue; 1 blue, 2 red, 9 green
Game 100: 10 blue, 2 red; 7 green, 20 blue, 9 red; 8 red, 6 green, 2 blue";

    let mut sum = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let game = parts.get(0).unwrap();
        let game_number: i32 = game
            .split(" ")
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap()
            .parse()
            .unwrap();
        let sets = parts.get(1).unwrap();
        let mut possible = true;
        for results in sets.split("; ") {
            for number_color in results.split(", ") {
                let number: i32 = number_color
                    .split(" ")
                    .collect::<Vec<&str>>()
                    .get(0)
                    .unwrap()
                    .parse()
                    .unwrap();

                let temp = number_color.split(" ").collect::<Vec<&str>>();
                let color = temp.get(1).unwrap();

                if *color == "red" && number > 12 {
                    possible = false;
                }

                if *color == "green" && number > 13 {
                    possible = false;
                }

                if *color == "blue" && number > 14 {
                    possible = false;
                }
            }
        }

        if possible {
            sum += game_number;
        }
    }

    // First part
    println!("{}", sum);

    let mut sum = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let game = parts.get(0).unwrap();
        let game_number: i32 = game
            .split(" ")
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap()
            .parse()
            .unwrap();
        let sets = parts.get(1).unwrap();
        let mut largest_red = 0;
        let mut largest_green = 0;
        let mut largest_blue = 0;
        for results in sets.split("; ") {
            for number_color in results.split(", ") {
                let number: i32 = number_color
                    .split(" ")
                    .collect::<Vec<&str>>()
                    .get(0)
                    .unwrap()
                    .parse()
                    .unwrap();

                let temp = number_color.split(" ").collect::<Vec<&str>>();
                let color = temp.get(1).unwrap();

                if *color == "red" && number > largest_red {
                    largest_red = number;
                }

                if *color == "green" && number > largest_green {
                    largest_green = number;
                }

                if *color == "blue" && number > largest_blue {
                    largest_blue = number;
                }
            }
        }

        sum += largest_blue * largest_green * largest_red;
    }

    println!("{}", sum);
}

fn day1() {
    let input = "99lbqpxzzlbtvkmfrvrnmcxttseven
q7cnfslbtpkvseven
6threezlljtzcr1sdjkthree4cx
21xfxfourmzmqbqp1
lkdbjd5
8three27
21three
3lqrzdq16
49threenjdgrmgfnfhcgz
fourmsmjqfmbjvtwosevendcljsdcstl3one
four98
4sevenfddxgcvdgx
dffmkvmhhdbzjcgrjc5132
eight4one31nxlnrzvtfvrkfvgbbqmvff
mdmvbhqjt5rkfpcnfvzhkkfbjvh8three9
four32
seven6pljhqnineeightjjsvnqblk8eight
6glzfour77fiveone
ntvhxqzsixxcrfpgstwo915onevxz
81four8xkttczb2vj
six8flfzdzl72eightnine
sevensevenzsztgvh4sixbzltzl
2fivehgrszcrgc9
four53ninedrjllgffrfrtjgggvknine
nineonecxpzzqsptc7nv9pck
sixrqqfkcjrqkppvqthree1
1five72cxh3fivefive
75349sixnhxjpgtnhqlf
9963onefourthree6oneightq
51fp
foursix2lt7one1sevenone
fourddcmgbvkm8fivethreeksqghch65ztdtsdmpvk
35jxmgctsevennine
mbbkjxfthreetwoqlcrjnlvkrgnine41vkjknnmtv
three7sixtwohpghsdxleightthreetwo5
twocbtbkxhhcdrctkc14hlmdh
4sevensix3ninedss8
snqhqmffonettwofourgdkjmbjvjpgxxxpzkm8zfpfcgj
3xmpfmnlz9fourthree9pdnzdlcsix
66fourone1
bdkfdqvkmmstkhdsbbl5
48nfive
twothreesixthreecvsskxq3threefoureight
1vkseveneight
gskgpsix4lhrtwo88threernhqnnfqdf
nskfvvncxfourninehkrbxpgdsk4
vlm3qszxqnnzz1ptcvdxnf
xrppvsfourfive3sixthreebvbdkpckgheight
pckngkbfour3nineninepqcxvsqspbthmhzrp
pbtndgpfkhpflpj39zkpjcnine
954fourvrmftt
2xjrkdb8mzcfkvbdcx6
plt462
mcmzfzcdones2seven4
8chnpnqmbx946eightkdjvhkkbtthree
6zgvqslsixnine4fivethreegqqpjz
9twosixseven4
7seven75
four7sevenmdj8
6r219sevenpcvfpmfxxl
dvxjblhdjqttxdfourhhrgdpmvvone83
one3sdnfone1ddjlzhgninethreenine
nine3sbj5msppfonetwo
14tgmqlnh
seven5xjncjmggjppmnsx7fourdzxrsck
j1hphv
61threejjlccdzvsevenjqpbjsphdq
pjmsphdtwo5x6vjzmvtkfhk43hknqc
xfpbzdl6
twosix3fournlddsqmfbpslstnfiveonezfgoneighthtl
828fivethreetwo2
fmrgz5sxrrd
one88cjcvxddjmczjznhpnvsix
two9six7pkgttqdnsbgfdcdsixmskjgfvvlvqvdx
six7eighteightq68eight
seven4bsrfpxb6threesevenmgxfmfive
gf74fourjdflkdtrvmljskxsix5three
xsvrhh25
2sixtwofivez813four
kbkclv3onelmf4ntxhxbrppsixsix
8nine9hstninezzbflnjfxrndcpzknk
1hjlnsszbmtjtwo
2one146
9jdpnzgqrf
1qfbhl83threezxcrdfzcbbqv
p4lcmztns33
xmcsevenseven21jjfphz7sevengxlvfrrj
jfkmdmrfhv8
73spzlbvnxsqsvgpbkl
kkeightlrnj4three
4qxzfxjdmrddlhxfive173eight
jhmvxpbeight681gj6seven
mqpsxqpxj3nine9ss
fourdjqll2vbqbtwo
28fivesix6one
pgblvrqlnjfdtvngfbzpl5njsmvshn5tc
eightvvh43nine
dpxc5oneonebdbkheight
pveightwothree2kfzpkks3ljxnbp
vknl4xptptjjp
4twotfour
182eight
ninesixqkfbhgtwo3
twoseventwo4fivethree3one
sevenoneeightnine4sgkckjszmp
six4svkqftsrdzk
bhnbm96fivenineszx8sevenoneightkdk
rjldcxl8cxkpsdsdpgbngqeightnine9
5two6bdrksqgd
threestrhsdtmnc998xfdtfg66
bknkttsjf6gonesixeight
nine6one9jnqf1
qkrpkv95bd11two8
dksqfpmb6ttt5twothree5
xxhrnpjsvcq7nineonexjkcsrbxr6one
hsmdgfvbsk76four1sixhfn
ninefiveeightrflsdc4one9
four73915seventhfmjjqgxjzsnxvmnl
seven9phpseven9ftwo
59121fivep4
gg3eightgdngqf
4zbthb7
rxrhprtfb4ljdzqfmhgbzqzmrvvj1eight
zkvmsrvtk3two4
ckbk8nine
tbbkxr844lpkgjvmsix
onetwo8kbxqgvsevenmrhqndt
ptvgfn6rnpfkxcc52
three1vd
fivefiveeightdrljdrljxc9bjpbqseven8
fourx7mlksevendjtr1lb4
6ndfqts3two88
qh976eightvfch7
five9xdthree1seventhreeseven2
xvfmccmmjcjjsm5
ffiveeight579
97hcfbrhkfive39lzhgjddsx6
388eightkrmdktwopjdlpfmfivetwoneql
eightonenine4
bqzpzbzkbs7nprbdmbqseven8kzr1pflnine
9eightfourthree5four
cclc6
13one8tkfrmvcjfq4sixjlxj
dtrtwo9foursflc
3six4two3nine
7vkztxh6
xthtrhfz83six
twofive56
nineeight5
one5qmhmvrkktjmvhgxx
4four1
8one2dljjrvkthreehjdsbpqgtbjm
foursevennine51s
1nine8ddhdnbheightseven1
nine5five2375lhphjk
ftdrpmmzhvrxcz3vfqnhjqlqxdcrxeighteightspmj9
qbczvsdl449six1
seventwohpknrrjtvfhpxpzb3sevenn2
jnpxrqrtwogqsk4
three9fouronejjlrzqdrxkcqnd
ql871three1rbhtszd
xfbrnkzshqtwo43twohhfqdddc8j
ninedgj6
two3three6ninethreefourseven
4bfsl1shdzeight3
96mcdgv1
4fivefour4
sixfivetwo4pjbkcg
4onetwotrnqlgxgtxxrgxpgsevenddjfd
three19
jbnrs5bgmsixeightxzjznzkhpvsix5twoneb
7sixpzhsbnhrrmonetwosrjbt1jcxflk
74onetdh7
5one4cmmcmjnl9eightkvhrnkgqjstxnpt
seven9xttdmzsix9tbnjfgmsgfoureight
32five4
7five41lpscgz
48three
hddfhppseven2
5ninecbtfxkfrtccfdzplndjpseventfhqkmcm5
65ninesqgeightjt7
four3csxzztgbrninesbfzf
2bcnlphn
xsfmhnbdrj31828
ninezrvbf717six
7hlbhqxseven
2jrvfr5lbqzfjgpdgfourthree7srmq
6sixfive8nine
eight29one
onefour5ddgcrninedgdkzh1threesmcjmntnhh
gdg18ntxdpvvxcn7zk74
5onesixfnppbf2vxxbntdqvv
kmclhrb2five49sevennine3eight
6jdddgdjbgdqmkktf
26jhvtscmrjthreenine53mldd8
mkdcpnbsevencjfm21twotwo
3bqzmtnjhlbdszlrvpmvcgclbgfzljk74
8gfbmnnlltjcmjkmmvnfsgph
eightpcggqtqns2
ztzrvxglv7three5cmhnbt1five1chfk
9eight8cfzrsixthreevvvfour
3nine824ninefour
nmrjjp2fourd
twoonemgdm67
foursdmljtklzldsevenvbqpthree917
3ddzjxlsg69nine2eighttwoseven
6btlmdone8eight
7qhmpdeight
dxlb1
zp9bqm4176seven
5krrrsix6
9seven4ninexsgrqs6
9zzfj98fourpjzqtk9
svsrlmggeightpgmprpkpj486twot
sevenfourtwo8sevensix7
zqspkktb831xjlhvfourtbtvpzmlbbt
2gfffoursix5
8ccqxqm4tsdpvvxpdpcrlmktmg
5fivesix
7fourone55two7fivenine
9twothreethreeoneone
xtzc64fourzkpcqjg7twobslssjln
38fivetwo716
hkxrxtdjzdzqnrzxfzsix3three
onetwo8
ppbrqlhvqpcfx2
rhlbtdddqggnmfour74zcmrkdthree
threesix3seven1one
nine6six9eight
mzmhvmrmtvsk5
2qgbkrrgeightfzflbhpxctdpg39
61two6
2rzbdfmbczxcgphmonexmntkcmdxxpg6
b6threetwosix791
sevenmzlmcqxdbfmfj8ckchpkgxgmtmnzcninethree
jvrpkltm9n9p
two3bhlrgqjxbc6crzbvzmz9nqfdtztvqbhcrjptdvm
jdjchvpkcrfive67fivegltkrflqsbktkc
sevennine99jpkqljhfztbqrzm2twoctrgm
rcmlkj5
lltwo4ninestwoseven1l
65threerzpkhjtzxl3jx
ninesixninebdlfckbp2
fourdqqklkgcsbtwobj5
threesix44six2tctgp
9zczcltwo
vr6bmpl
7fivelpfnzgct
ninethreesix1twocqfpchfour9
1jcj81tkklgrhhjt
pntwonetwotnpnbmonennfblctcv91nqr
jnsfspcg45xkc
8eightqbhmrqc
five5threerhxffonemkbnine
16hqlvzjc
3mzjkkr
kcsrxhsnmqkz3two
9ftxzstqrdjcqddvzdxljz6sixsbxlztnqgthree
threeninesxt965six
4threenine2417qcljfive
4mqzxsnt3hqbhjj
r9ctkhvtl51
47kmgznjvcg5eight
one5rv9ztvpt1
2one3htxp3three
74six4bkqsjfgtdt
two2onenine9fourfdqqrmx4one
rvltwo9grrhsvgzr6pmnvrnlxb
ninefourrzgfgcfcvv8two8oneone
1zg5seven3
six91xgqcs28
5threemxxntrml
3hjltzgdgcmfmstvgfivenxfour5eight
five1dl8six8
96lkghnglthree
7sevenvggnkkjvzmjbbjgjhmtx53
9onetwo
6tmltdjvsevenfourfive
ghqkskqmbnhnone7dphgvqjzbq834
nllxconethree1
fivez5lxtfpjxpfdtwo6twosevenkhtmqbm
eightone4lnhlq3639np
8cqcjfxvclskqqnbgq8
qvsixgxlfsevensixhqtgmnmvk6
twopfccg6nine4gdhrzdqrbm
1hg72five
kqrr96jhsfxgddncone4
55xsfk4nqzqvthck4onerldsjx
ninesbcqfdlvl1sixeighthdnhsh3eighteight
rrspvpclvl7eightthreeqtfive
twobfjrmffive6three9kqmqmmps4lrjzjqt
fivefivejjxcmczlsdeighthdxbltgvjgthree8mp
one46mhghdxldbfiveckljksxm
c7fiveonesix
sixnineseven8xqb48eight
2mqxkvdmhgbnx
sixkrknqflbz6
6four9twogonesix
cbxrknrrddnfour73ninexqdhlql
rjtdxkjfs628eight
xchpj2
fhteightsdzsjqvhgceight43threefive
426jqvrxqflhch9
sevenmj3fiverfqzhhpvtbkrqglqzp
2nine8fnz3five7xjzrp
twofiveeightfivedcnpv74
7fiverkzrthree
gfctwonesevenhhcsvjqfourpfxjmthbcm5mttmfrb
fourjc9threethreetwosixpffp
sf9eight5fivenrlxfkjgq9
hd48bq
five21sixone
vgkgfvnblrdbeight245nlfnnhfx
3fsdrdmvrqptwothree
ntcx82fivesevencvdkk5
6gdmmvxcvp5sixthreecjtfkgnpmnthree4three
xpkthreethreeeightxzkdv7mgrbzxhfive
one98
5foureightmqstc
twothreejrhffour9twosxcj
twoseven5five4ptz34
1phdclsix
3chbfpvfzj51tbhpvqhninefschztfbdm
113tqmjkzjlgr
kjdrfrnzhvone6bhv8zlllc2
four1nine
3pkxz8
four1oneseventcspbzninenineqdg
one9lsmfghl6pxhbmmzxpjjthree
szlqptvcvskkcgxxtfrfivefive28pccjhzz
7sixjhhzhgh
onerxgnxcvprphgpzsixphrkdd97two
twodhqbmrz11lg
5ninectctwo2ninenc8
hntt5
chkjc4twokc
threecsix62one8tgnsqxxvpjsix
fhtjrljsevenfchxccpckl8
nine61sixrqktdm9
threepkzmnmnmmngxrcq7
4dtfivehdvcknmkj4sevend3tx
njcvzpmp8gmgvtoneninecnhgj3five
4pvqncghvr4pmlone
2eight4596bs6
twosix93fclbgfive9fourxnqdhkg
8onektghncz3dcklzvbdtwovsnlrcclpc
221threegtmppngzseven
jrhvplk55five7keight
4eightktndbfzfmskcjlm1
fskbxxvvsmsevenrzqrcgninemscc8vnvzseven
kbtsf6sixczlbqzrjfm9oneightmqg
vvn3threefour6sixseven2
6jfourzkhlllkseven
2twodxxhhns4gdjqmztnine
sevensfxztmfive7twofourqbzlzkjc
vqcblfvrgfourfiveseven7
6nineseven4
bnfivevbjgfzcctrhncmmnpfzmcdt4six
two8onesevenonecmtddxnfbjeight7
4sixfourdgmstmsj
7eight97kzxxdbdonejtsjqgjcdlssix
vtvdone2cjzcjltcj6six
ztwobmrvcbfsnine4one
jjblshvzsevenfoursix1ndqlgphrbbfkkcmstlsoneightfjm
5seventwofive
onebk4
61bgcdfjsevenhhcjjgdqhr
6kdvfpsevensevendg3
61gslvnlcjlsfgdsflqp1rnqdgktfseveneightwom
rvcmkdtqqglcvsxkntfour32bdptgseventhree
zvsbhkftlpgntvfgfxpcztm9fournmvxjnine
four91nptdgzk8rnmqc
tqmtfsixbcxspjscnine2
1kvmreighthnine2qxvrckqvgd5
dcpvrl8two1fivec698
xqg91snbsslnrptwoeightdzq
gsnvsix4vdmszmjfourfnrnine
9nineeightthree51gl2
7sixffvf9two
bbgqpxghsb45seven312eightworg
fournineeight45five4fivebrznpkz
dhrsrghxfivejdrqfgkdlxzb869
8nzxvsmrrcz1one5four
3mmlzvzqptttbbmtgthfive1nine
one9eightpzsmjvnptwonemkf
fivejkgrclbthree5
5fourtxxxvfthreelxcmghhtkqnrqzvts
1tlhcscbd
99six
53sixthreetwoninedjsvdfourone
fivefour7pbponectstmp
six5tnl164htsv
fivekbkjtjkxfszcqvppqqxdtwo29
42threexpdfive
9892
lvlkqlzvsix7gbxhpxrdfh2zcgccjvblhpvxqshmbbjkpgpd
two7sevenfourblphfkgoneone3
7onenine6535
59zv
347threeeight
8five49
73four
heightwosix99
1foureight
soneight1xzmfs8six7vpxcfq
nine91
pvxtkphg3gzgvqlfk9vhjvqmgszfivegd
seventwo9six2
2sqp
3twoeight3
8slnsxtteight429seven
qfjtthmthree3twothree
txrdqsp4oneqsprgjnnsc
7kqgv3five1
fourninegrfz2
seventbbfrvjqlkvm47
8pkqtpxfxrgpq1
kqvseven4n5ksixdphmjk
17j
1m3
ninefivefour6
jrdmvztnncnhnp8onefourfive88
kbvninetworblsd3fivesixhvhtxvgt
4ninesevenkpfgcddkninetnhg
9rdzsixsixtfbgzmhsgconethreedfxqqvv4twonem
kvbeightsdtqrhsscpone6
7fivezhxkkxlsonenine
sevenone7threeddmjmsjrhprn
eightsix5
4bjmfskbtc37seven3eight
grxbdmppllthreeeight53
one8jnhgjpdbseven8bc5fivenine
jvlmsfive43gqmpzseventhree
jqj9twosix
eightseven9
fourninexdzbnqsvkdvbbkb6
pttwone2nbone7gnpbllbhp8
sxpkrnshj8fivefive9kfvgjpv
4threepkvz2shljdn9hgrmznine
6seven29gnhtvshpks77fourstkdf
bmnfthree2threetwofive
51
jghxzhndrkfourtwo1
jspckbhzfcqsbone26mfhkmqztnine
5fgzbmgfivefive
six18sxvchr
rv7fivebdrtdl5twojt4cx
twofourthreetwo5onejfm6
threesixeight2
9jrmdmzvlgnine6cjlphdnine7
9fivembvhfrlnzbsevenngp73ninefour
sixxtcgjvssevenmfiveseven15l
threeseven6
cknkdqrnxrjfbn818vstmprqbts
4zfmxjxlkv7rdqhjfourfoureightcgvrpxfnrn
ghzmpsgzrkztp8s4zl
ninefour78rhtjfrqvh4
one9qzkjtsix13
sixfk72jjnrninertbzheight
rxhvxzcp366one
24eightthree
teightwofprzdscnts4nv88
eighteightshqcbqzxmbktwo54fourpdkf
sixfhvgkfourfoursjxnstgqnjh2
6two1vtnqbrhqjbnkm7six9six
66threeeight
lbdkggsncthree315
8gqsrmseven3gvvxq99
sltgzbgbmg1
9twov6
shhbqkcc71threecmkl
cgqtpsjkglzszffcnineninesgvj1
fourhqnlhrrsbrsevenkrgffivefour6eight4
twojdccnhk6fivelddgrppkldtdlt
4dlkn5x
12sfdtvcqztxmtvvkds
44one6jxjdbsjxpg
7sixfmbbfxnjjhj5qkqfvfntonegdktrfl
eightfourzkdxgqn8
1fourthreetbvtpphj4rmrhlcbxbrqfdxszfour
eight42onehpvrlbkglq
9five2five5
bnzbccq26mblqtjxlsbtdvm
nineninelnknxhbfk4xssrlsdmsixoneltjseightwofzf
8mleight7zhfsmsmpdthree
mjcsr5tsktfpzc2nine7
tmlxbkh79four7
fiverzrfvlcdnmkn46onennxbbvn
threefournine9
snz6bcgqlhx
4ninevqjlbfklgz
8fsvsfiveone9fvdfiveg5
eight5dhcgxjts
ljldjmfmgtzclfhthkdtdthree4
qrlmdjmkvvtwopdphfpmdd6fourxkblfqcx5
62glckjgdvnpfourzlkphvrjffive
nltxkzk2zhmqhfqq
95six8xflfzhf3
zsfvrpjx67
two4vfivehglclslddsix
gssccpnhjx5128foursxpgrgztdfour
87fourpqdrxmvqdmxfrtzthree
one85fmkj
5rtjzsevenfive1thpzqxxm5
bone78
4three83947
sixffkfmhzfkksevennllffhjvkxqhpjjtfl1
foursixxvfst6twosbvjfvcb2t
fgzthreekkpffive5four
685jfttdmjq
t4
xqeightwojhbrrrqgdtbcqlhthreesixsreight7rxxgqntqdqlbnm
stbmgqjdvqfour4
dfour11gfcvx4nine5
eight4fourseven5five
six3seveneight
four2sevenltvckjxjhjfslsvgpzxffivefive
27one5vdtkjt4
fourzpr1gxgone
gtfive5hbqktbfour5g2tk
nine2pqhqgprxrg9hgjj4
twovcjkcsqznsix557psh2
1fkpjccjffr7nine3gxzk
5threezzbbvtcbvj4
8nkbdhct
8eight2
six17ninetworpc4
qljgkvq23eightfourfour
onebblhxbnlhztgrkchlbtwo5
sixthreevmngjjgpfourrrblxqvcl22
mcnksthree81xrzrrrvnvdvbzfzlrgseven
chjxhjjxthree3
mrmxzpbxnh4sixrxskzvnjtlkjddnrpkbjhslbxjbkq8fsix
2kgfmzeight
plvslqzpbk7
xsfq49ljfts
seven2x9hpzbhqzpffoursix8
5999one1mbnxmrqcxv9
eight52prffgvvgznineseven
l8onehqzcf
sixkzxvqdgnbbmdtrvhzfouronekmklkjp34v
98144threethreebvcjpllggz
nine7twofive9sevenszvdrq
jnvcprvbgtfourthreed9pm3
tszbl9one5dfmsmkmlvfrlf
537tmcrdxp
sfcqfrrcqj26
8fourfoursevensixmlmqzggmrfive
kbmbxz1onelqtdxxk3
45mxksixfour
7zncmh2
hzoneightsixfourzzlkmrnzptllthree2tpdjtsszxnjdhkgv
ztpqbd68814five
92gnzsvldmvhhzb8
ninefour2sixseven1seven4vcnkgklh
sevenfsjrdhclm7six8nlrhdpplbjg
sevenlgd86
pg75sixtwozk
734eightone5gssxhffscq
5sevendrxvkmfsjkgg5twocqpqlhksrp
seven1cktfl
mrs7
2foursevenninelpzxthreetwojmbvfzs
2rdsjl1three1
xrkztslbkgdzgjjtst5six5fourthree
twosixfhlldtwo2sevenfourknbfgnnjs
nine699two9pp1
9kdlfour8kzfm57two
hlmscnhk1cqqzqbsxqglf9jgpjnfrjczeightspkkzrdqjd
72six7mjsxfxtz5
3sixonebkfgp
fivefourvslvn2six
19pshkt
739bzfpltmtsevenfour27
fvps6pprtjlq5foureightworm
qds3zdsgzxvthreethreeeightdqjbtrfjbbj
nine3148oneeight
nineggvmffthree1fivefbkmmfvjkpcgsxrfzg
7vlff9msmx3njb76
hsthree4jfpvhxnceightkfseventhree
hx3seventwonineklbq7six
38zeightjlxj
jdbqgmsix8
seven8njthzbdrtninexkplhnrfourone
tnhhzhjccjdtpleight3onegsevenxkqbmqzx
4679scb2
twos334fourfivefourfour
sixseventwo356
xnmsevennlr78jlfrbgb
four7krmgzcclgf6dteighteight
94mzfhdk
7two43threecthree
lztbgnscq7ggeightsevengtjpseven
nbcrvpljfive1threefive5seven
fc895
eightlqzzlgrxv9gdfcrpkxkhzgbjtpcgncppm
8cfjbngnd6371threedczkfqmptxtrqt
6jblvpxskcnine
onexvkxthreebcvnnrctsix1oneightsrd
two66fourfourdnccpl
3fourbqznm
eight9lbz3eightfourmonethree
2qpvlbbb
threeoneone1mjhqfive
2threenine
two6qvxnnqj91eightfourkqxvhq
rtsftmvb14
kvzptdxdfsrm5four5three
sixkfjjbccbeight3eight
kjskjrnbfs29seven6
68qxpjrz
eight4cnmx9four9eightvbsvvsix
6xvsptbqcsxlcdzcnzrninehhrjqjsixkk
seven7vknjphhfbs
rjmhjgmpzk22fjmtmtc7spgkfkqgcn
three8ftxlktggjn2rfljxmlg9
9threehlvtmftzfiveqghnvmtbseventwo21
pceightwopbpbj3two2eightfive7
lsdbf6five3nddpcnnine5xvqx
2sevenseven86lldmhf
8rqhnhsrcjnfzgpcclrmnlvbphrtgchpls9
kxeightwoxsrvcbzhhbpdlrt22fhqfivefptwo
bdseven2hnqccjqgxjvjk86xvbhddlbx6
zszz4kqtsqjfpqmcxbndmbdseventhreeeight
fourfiveoneseven83twotzblfpldfq4
tgbxlthreezdfspjsnhrmrqxqj6cplcczt2pgsbfczn
jzdkc6xfmhpqstvzzmvxk431
7eightqbxcnsgxm
5sixeight
6sixvjstz
4trxhdlkzqvjpzgpvfp1
sixqhghvddcdn7blhptxp5htcf3
nineksjvmvc2
41krm9
nine42sevenfive2
zfivepkfqrgpkmxbmjbq6tgktpvnjvdjzsd
gssrnnqmm7sixfivelklmggxgkdtlmlzqp
64qsevenftmjzgs
6nctkfbskghpqr7
8nine3
hkjoneight6seventhree9
11nineeight9nine
845fouronetwothree
pfhb14v
three21kdgcplbzzbeighteight
4cszltd7ninerrmhjqmtrprtccpjnjgdbdtwo4
five96
2lkfbsnq64three
srltvpbgnxvlksmfzpj2
four9tpjvhhpsctclzpnsr1fourcttqvng
gpjxncqsbp46vb9tqgnninetwo
four1four
6dtklvddhlprphffpnkrksfseventwonek
eight4seveneight2onezfbnvjplvl97
7ndvfp8qstcjdgzcfcninebj
9beightbsbphgrnq1ninesixsvthrx
bpxthreezkhjleightsbxmsdeightseven4
fivemlfsninesix1nine6six
4fivethree3qtqsbscnc
four7dsix3kfhrrgbbnlzdfbgxsix
hqckztqxgxeightkfmvvjg6tp4nine8
tmgfmcl2twonedg
fmvvcsix8nineninepqdrcmhdzsixtwohcnrss
nine258foursevensix8eight
8q
pqjqrxnine2one
9sixpbxr43sflnine3
4onethreeonefoursixonefour
1vnscdrnnfpkrj8mndxbqdrckzgdpnfdone
threeseven7six8three3six2
jvt24pjtnxdcpsthree
7nxfpnfzrssvqqcnrjnine47
cflhgfournine368
62srjsxgr
mnsix6fivefourjdnqfgjvp99two
8five6fdvgctwoeightcsmspmxmbgjqzjhlhb
seventgcdvchsfivefour9nine4seven
bbzvxfvmxhqv26rp5
2psmsflpgqmb
eightnine4threekgfjmhbkhtxgr4threekxnmspvbfs
seven9two84phzrrvftgpt
xgpfkphfchzptzvconetxcp3qdmdn
five2fddc8hdzrzgcgdtonesix
one24sixtwo
mxfltrcltqjmmtj4psbf
jbrskjdtthvksk9mhfxjgdjt
four96njn
five8tqltpdxrklninednqkmgstlptpjhqvklnjhrvpzvpfr3
onecsktmkt9
seveneight1
nineeighttwo1txhjjkqzqtnine
37onegfcf253
4ninethreepndgnfqpvqzxbxkpgp4eightwozg
tfxnx999864three
sixthree19
3nineonefive
rpht1onetztgngmeight
9928two4six5sp
gmhtwooneclqfdqvfivefour5
two3dkkfive
7four6pklfxjqhgbvnpxmndsixthreetwoeight
5eightthreeseven9
9fourseven5pmtcdpfvhjj
24fhbms2sixtrjtm
seventwob99
3mffcdvdqsixgxtbrxqddkrzclz92
5fivenkmtgbsnsixeight
3seven9hcbmcnjvqfour7
bvspr4jhjlnddp3eighttwosixfour
kdvh5
seven7hvcmdfkfour5nine
1ksqzfnmhfqjjlklptmmpvxnine6
fxp7hljzvndnd2mpqm8
86qxntjqmljdqpdbftqfrfiveninefcxzdqctbjppc
one62
two23fivesgtsqpjrkvlbrfknjgcjbt
29gkcpcfdlm3
ninehzvhg2one
ljv73tpfjvbhlxnpxqxhfivefivezxtzhlfsc
three7637
rnmkzqgtjpfbn3nine8nine9ttcsdbr
jqtwo7fourone7dclxtnlnthree1
872fivemjghskxcxmninexljstpvb
9hdjtfkqsb8skkvpfpgqlctvlz
nqnltrnsevenfbsixmnxrv87ffsjn5
fkxzbxsbbfour17fivenine424
ccbrxlskjzmrclgchlxlcpkck2
one9sevenjgxrk33twonepjs
5eightthreefivethreetwor
zxqdnhnk51
two39xctxpvsmfivenbtsmpcg3eight
three52threevgj8
1twom52
3pthscht
8ninegx3f8
lzjvthree96jgfhfvsbv7qfour9
threenineeight722ltvvxhdczjfivedcxbst
fourgvlbxrone8btblnlbkmx51
one5nine93fzxthreebzchcqjbrxxct
3sjbbtbpkr1onectngpqpdfp
5xdgpgpm
nine9onev952
fxc9seven
fiveggdqqlspfoursevenfour5three
7ninekhxpdgfive
3fmkhhbztk
737
3kx2sixgrgslpfvvlfourfive7gbk
1rqjxnfmzkvxxsixt
fiveszkkmrtrctjgqvcfbg5r5
6onefoursixntbbrjxfrfournpjbgcx
sixgpbbbkhxdj4skfrvnnlmmhxcpfnxfive
two37hjdklhtvtwonzhcqrbfv
eightsix7
four9253njcqvvhtgs
fourfive3bxnvck
ninelsrbgctxvn59six
3jzqb7six69eightonenine
73nine6tkrqsc3kthjdmdtmsixspvvr
8691twothreeeighteight
8ninetwo7nine21qpkbml
3lttttwonemh
8eight9onefpscrm5
qgl38onevqjbfldcg
dbcxgcfsf23jbb6eight671
18pzgfqjsfourthree4bvqtxsh
fiveseven7mdgmsbdphthreefivedpzjbjc
7onemmspfpbfbftcpgvxhmh944cvdvlp
96hpgcqlp8six
two5jrrsix4dktmzkqgvb
two93fourninekfkkgdxbvk
93onesdnxqtwo
jxmfmmvlxzseventhree5sfconeeightkrzdcvnhpvmfnz
86gkxpqspnxs
seven9pblxnnkkjffournjlkgqmonedxqseven6
jtddqv7nzpsixseven
vgjkbfnc29gskmmjmgsrbvghgrbtsbrdglszsix6gm
gjglfour8three7five
seven8ninetc4foursevenxfxlvmt
sevenone21nine
9onefouronevqrsthmlone
8threemsrbrchmk22fivepjdxprpvplt
rsqkrlnfpsixfive3three77
twoeightseven2threejzgzmzzgqdrcmf
three8lsctcbnhgkpr3
4ktrtzn
vqhvfxrxhpdgqhcrrczjlmhdnlzseventvtrrktd9twonenqn
twonineeight7
67sixeighthvfkjhtj5
644
onezdfz6
6qmcd9ninenineninetwo8
four7nine7
4hjnndn
1fqfqdqqxdnvhmqcp8
rrhndnbpjjzhclhv9ppvlbtxklzfivejhxzcbpmd
six7526
8ninefkhszlp2
fivesix34twocg14
3sixfive
2nine3
2hv4rrfh3two8lr
sevenrqbzmkmcn22
56vrglvbcdtxxrnjrlhpffr
mfddxflkqd4six
bfsfnkqxc76rtpgss3pkqkksrcxnine
83fivemnjdfnk85zqmssgfffourvgqjbn
822
dvxgqrbjpnqvpsfthkz6
4fdlzcxeight8crmrztfdmthree7eightfive
two9threefourtwo8vj2
nine1jxpclfhj
jrgfxlgblzqnr9mnfrcsixone
5t3nine5
twogxssevenoneseventdc3
rvzcbtwofour4sixgfzk
six22eight81ninepnscnlv
eightfiveeightsevendxx1gh
7nzlpbx864g5
hxgdrrnnsix1sevennvmsvdvccpmfive
1threevz9
36eight4xvnrghgjf996
nine8mmtqkffkthree8xpmsbksix
1fivejrgqrjjnql2dvmcqdbjbsix
15fiveseven1td3
kmfmbhcmf62five
36dxfoneninefivehrr6
four54sixfzhq26
1427qjseventhreefiveqhv
eightghnnlddqdpm1
9jbcfdjzjxreightlcpmdddts
6ninesrdgkrfivefoursckkrfrpqqhgzeightwoj
xmqsnjgskfour8eight
9l8one
four9jghzbrsix7seveneight4
9tgsvk
sxgkkbnlsbnbbc2ckqhzgdlsbhp
6ktwohsvbeighttwo
xgfrrnrlkgdqfxdtwo9fvthree
5hqdvsdnrvr81sixsqjmbdls
kkghseightsevenmtbtvplcjnqtscx6
rveightwobhqtmjhsrptpzkbv3ninem
3qpgbd2fourqdsnnbgmnsqgdb4
mfive12
52xqhpfdjgmeight13tcn3four
6lkgjdj
rvsvpt144lrtvrtpvd
8fourfive8
28dhzrtfcjlgchxbthree
vvsixtwodxfz5six
seven7rcp7
threethree136gts9threenine
9477
seveneight8gmhthree8
nn633sixzxcxthree
two95rfcffourtwo3
76nine
cvhlpzsbmknkqpgsevenlkzvm7hnznjsbszgvxrmdnn4
pnlqmxlmvkkpdgktwo1four41one
4hgnhzkbmlvkqpqqptd93two
lfsnfs7eightvdnghthree
shpndlkr7five1three6
sevenfourfivesckjjlzgv27four
xhtpxlqssd3eight6two6
48ckzhkk84seven1ggsffz
33rqthree6jvqlbzllz
4jfourthree6
jkhflbhqhtmpmhmonebfbmcmczbspj9sttmnvnvgfivekz
eighttwodk25sevenninenine2
oneqqxfkffivesevenzqhvjtgjlrninenine1
2nineseven82fivethreezzqfjm
mh4
3tjfmrtnff498
five7three7three52eight
jgrndnjckvc1
9sixgvv
six3rfpxrl9three1chczskrxthree
6bjqqzfqcxvnxbkpq
922
loneightnznrcpd6cpfsmclp
7five7qncxmfhleight
bhsgbsdnsixnine4msnfmonerjbpvkqf6five
4xrcrrllzh9two6zgcftpfqj47
six31two5fourthreeeight
gqhckrzrpltwo72znb4oneeight
trsxcpls3pmqqrskpmfivehdqrptvdtq
six3four87one
46two3
9fiveseven57vhmq
jkrbkfsevencnvzp89vhmsdcfcthreetwonedrl
9cvzvqfboneffive6jmnpjkvrjz
82xrqjdqchdbvhxpninefourdhb
3bqsrf
fourtwosixsevensix9threesixthree
3fourkgjbxbpsrtgsmgrjthree
ndeightsevenfourtwo943
471pn6
bpstsix84four32
gjkcq89ninegkckjpkz
eightkgvmxlzlhrnrdpttdqbthree117
vtdnlkqcg65
btwone727onesevensixkhbnkvtlthfjfive
twozspnzkssqseven413
7nttlqthreenspcdrhdpn563
six3nine
4twodkrlmssknsgfourkcfrxskpntnine
qgtgfxhfjj3pslzsttxpfive
fivethree9ct55sevensix
8twooneonesixjlszd1hlhbcgjf
fivetwosixsevenmrzjqjtfourmrk7two
pfnine6sevenone
3threex94znone96
79two2sixeightqmnbxcxf8
oneeight5
763lsdvlz24threed
four9oneeighthdcrzqlnvxj6ninex
7552twojhv4
3rzxhddcfxone553one9
sixsix8
7seven26k
86onethreenine1vlnmvghn
sevenglrmsnngpf3mfblll3seven
npdrbsxxpcpc3jzlpljsnlsrvdmvtr6one
6894one
rjzxs7
7mllptmtwoqxrpgprvqrstmvvvvm
sixone12
225
dcjzmcnbpptwothreedzqctmhm4seven3qhnbn
oneone2six56jsmdjqcznjtwolcjvmpkxlj
2sevenseven
7cpnpkcqsstfourjthreenbzvrdgeightsevenhfnztth
4phcrvcgseven4njn4
three9fiveninefdv
63ll
9sevenx7
9fivehvtfdckjnpvk
gpkhmlrxs5two39
kfvqv5oneonefiveonethree5five
2mrjctkmlks4z
8eightkrzkdxvrrjvtwo2
fiveninesix9
rhtnxkvthkpbqn4mtgtdnmmxmvmmzmcbt1
mtwonexdljqqgbbnineqrgbqnqx7eight
fiveeight8sfvzbpkeights4
8threesix8two8hszvhcbonefour
tpjtvpjgtnine2rbbfd
zrxjftfg4b6three37four
th5kktbzgmvkxhhcsqcrvtldnldml7seven
4txnqnplsixzptgszd
gzt5threefiveninetwoeightsevennine
jtfrlffv17268
qfour9six
49vrbpqkkzct6
14two
nine2drdcd7sixpfvsblcxqsjshhg
5onehfivefour8mnine
5qlthree6hbkktrpbmgninepqtxq
three8four
ftm67qrmsix
seightwoplhzgbvb7275
qnqtbdmqd4
onefive2lcrdjrzbheightthreegvc4three
87eight4fiveninefiveeight
1seven7eightfivezqcndfj5cmblgczd
seven6rkxsvbs529fbgbtclkjhcljpkjbgmvz
1twojxpxrgvvzq14ldngl9eight
qkg2fivemrlzlhxxzcmfive
one85four9six8eight
7sixbgvdf4841
hktntngtlfflzrdpfourninevlzpdrngvchg2
6two115
fiveninesevenqxgjrnk3two
eightthree988nrclmr
2one6
6737jfive
8four3
4fivesvntkxfpnqhone94three
7six1twothsd86
98pczqhlqbzjlvfnine68
xtqtwoneeightlvcjqfourckfour2nine
9sixllhhqhfivemmoneeight9
4czj3
ggdlxrrxjl1jnndbgbdninesbcfhd2five3
kszvbdfninethree5onevflrqffxmdonefour2
1nkpmvbf75
lnbgnkkfhseven5zfive2qcr
seven75xcx
2eightbzsp2pfg7eightv7
fourone29
4two5two9xcpkkjqxcfivessqqvhbbt
ncnqg1sixt9ninedlfgsqhnxx6
xrlsktwodnbcbonefvxxqgbrsdthree3seven
klvsv73
onezvbhrblrkzcrsevensix96jnpxjone
nine6chd4
bdvkqlrh9eight6eightninehq7
fivexpx1vsrreightkp7dph
3eightlrrlgck967
xcntwone4633sixmkm1nine";
    let mut final_sum = 0;

    for line in input.lines() {
        let mut final_number = "".to_string();
        for char in line.chars() {
            if char.is_numeric() {
                final_number.push(char);
                break;
            }
        }

        for char in line.chars().rev() {
            if char.is_numeric() {
                final_number.push(char);
                break;
            }
        }
        println!("");
        final_sum += final_number.parse::<i32>().unwrap();
    }

    // First result
    println!("{}", final_sum);

    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];

    let word_numbers = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut final_sum = 0;

    for line in input.lines() {
        let mut occurrences: HashMap<usize, String> = HashMap::new();
        let mut rev_occurrences: HashMap<usize, String> = HashMap::new();

        for number in numbers {
            if let Some(position) = line.find(number) {
                if number.len() > 1 {
                    occurrences.insert(
                        position,
                        word_numbers
                            .iter()
                            .position(|&x| x == number)
                            .unwrap()
                            .to_string(),
                    );
                } else {
                    occurrences.insert(position, number.to_string());
                }
            }

            if let Some(position) = line.rfind(number) {
                if number.len() > 1 {
                    rev_occurrences.insert(
                        position,
                        word_numbers
                            .iter()
                            .position(|&x| x == number)
                            .unwrap()
                            .to_string(),
                    );
                } else {
                    rev_occurrences.insert(position, number.to_string());
                }
            }
        }

        let min = occurrences.keys().min().unwrap();
        let rev_min = rev_occurrences.keys().max().unwrap();
        let first_number = occurrences.get(min).unwrap();
        let last_number = rev_occurrences.get(rev_min).unwrap();
        println!("{} {}", first_number, last_number);
        let mut final_number = "".to_string();
        final_number.push_str(first_number);
        final_number.push_str(last_number);
        final_sum += final_number.parse::<usize>().unwrap();
    }

    println!("{}", final_sum);
}
