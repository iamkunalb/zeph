import {task} from "hardhat/config"
import { HardhatRuntimeEnvironment } from "hardhat/types"

task('deploy', 'Deploys the contract')
    .setAction(async (args, hre:HardhatRuntimeEnvironment) => {
        const factory = await hre.ethers.getContractFactory('AIAgent');
        const contract = await factory.deploy();

        const dt = contract.deploymentTransaction();

        console.log('Deploy tx', dt?.hash);

        await contract.waitForDeployment();

        console.log('Contract', await contract.getAddress());
        
        
    })

task("expert-query", "Queries the expert contract")
  .addPositionalParam("contractAddress", "The deployed contract address")
  .setAction(async ({ contractAddress }, { ethers }) => {
    const expert = await ethers.getContractAt("AIAgent", contractAddress);

    console.log(`Using expert contract deployed at ${expert.target}`);

    console.log("sending a message");

    await expert.sendMessage("hello my friend");

    const message = await expert.getMessage();
    console.log(`Last observation: ${message.toString()}`);
    
    await expert.setResponse("Hello my friend, how can I help you?");
    const status = await expert.getResponse();
    console.log(`status: ${status.toString()}`);
    // Call the getChatHistory function

  });
