
// Example game
function passage_combat(state) {
    let monster_attack = Math.floor(Math.random() * 5);
    return {
        text: `You are fighting against a monster! You have ${state.player.hp} hp. It has ${state.enemy.hp} hp. It is attacking for ${monster_attack} hp!`,
        choices: {
            "Attack it": () => {
                state.enemy.hp -= 10;
                if (state.enemy.hp <= 0) {
                    return next_passage(state.return_passage, state.return_state);
                } else {
                    state.player.hp -= monster_attack;
                    if (state.player.hp <= 0) {
                        return next_passage(passage_combat_death, state)
                    };
                    return next_passage(passage_combat, state);
                }
            },
            "Defend against its attack": () => {
                monster_attack = Math.floor(monster_attack / 2);
                state.player.hp -= monster_attack;
                if (state.player.hp <= 0) {
                    return next_passage(passage_combat_death, state)
                };
                return next_passage(passage_combat, state);
            }
        }
    }
}

function passage_combat_death(state) {
    return {
        text: "You died fighting against the monster!",
        choices: {},
    }
}

function passage_caverns(state) {
    return {
        text: "You are exploring the caverns. You see a path forwards, but it is blocked by a monster.",
        choices: {
            "Engage the monster": () => {
                let combat_state = {
                    return_passage: passage_path,
                    return_state: state,
                    enemy: {
                        hp: 100,
                    },
                    player: {
                        hp: 20,
                    }
                };
                return next_passage(passage_combat, combat_state)
            },
            "Explore more": () => {
                return next_passage(passage_caverns, state)
            }
        }

    }
}

function passage_path(state) {
    state.gold += 5;
    return {
        text: `You've found some treasure! You now have ${state.gold} gold.`,
        choices: {}
    }
}

run_engine(passage_caverns, {
    gold: 0,
})

// Engine
function next_passage(passage, state) {
    return {
        passage, state
    }
}


async function run_engine(start_passage, start_state) {
    let current_passage = start_passage;
    let current_state = start_state;

    while (true) {
        let passage_result = current_passage(current_state);
        console.log(passage_result.text);
        if (Object.keys(passage_result.choices).length == 0) {
            console.log("You've reached the end of the game.")
            break;
        }
        let choice = await choose(passage_result.choices);
        let { passage, state } = choice();
        current_passage = passage;
        current_state = state
    }
}

async function choose(opts) {
    const rl = require("readline").createInterface({ input: process.stdin, output: process.stdout });
    const entries = Object.entries(opts);

    console.log("\nPlease choose an option:\n");
    entries.forEach(([k, v], i) => console.log(`  ${i + 1}. ${k}`));

    return new Promise(res => {
        rl.question("\nYour choice: ", ans => {
            rl.close();
            const n = parseInt(ans, 10);
            res(n >= 1 && n <= entries.length ? entries[n - 1][1] : null);
        });
    });
}