# Smart Meter Attack Methods and Prevention


### Introduction
-  In smart meter the data's are storing and it send to **database(providers)**, data here in smart meter is energy readings, power readings, etc.. 
-  These readings are stored in a data base and the analysis can be happen for future **energy consumption or other estimations**.
-  These data can be hacked and modified by attacker on consumer side itself, so we need to analyse the false data for classification in database.

### smart meter attacks
-  As with any connected device, smart meters [pose some security risk](https://www.information-age.com/smart-metres-vulnerable-cyber-attacks-123470837/) to users 
-  smart meters are still considered a safe device and are relatively well-protected from attacks, but users and providers alike will need to stay on their toes to make sure they remain the only **ones collecting information**.

 ![Attacks](https://github.com/shyamprasath18/Researches/blob/main/Pasted%20image%2020220918160026.png)

- AMI Main componenets for Data transmission

![AMI components](https://github.com/shyamprasath18/Researches/blob/main/Pasted%20image%2020220918160246.png)


- Electronic devices in smart meters are vulnerable to **invasive and non-invasive attacks**. These devices are in the untrusted field, and in physical proximity that can be used as the point of attack to bring the grid down.

| Non-invasive attacks. --> • Are **side-channel attacks which exploit weak channels**. – Hidden information may leak in the form of physical phenomena Example: Power consumption, electro-magnetic emission, etc..

| Invasive attacks refer to **attacks of physical systems where the physical properties of the chip are irreversibly modified**. 

- Different kinds of attacks are possible using “standard” reverse engineering techniques , like directly modifying the electronic component architecture and injecting some malware.

Other common kinds of attacks are majorly used

```
- Denial of Service (DoS) attack
- MITM attack 
- ARP cache poisoning attacks
- TCP SYN Flood Attack
- Ping Flood Attack 
```

- These are the attack can able to compromise the network which connected to smart power meter and we can tamper into the network based on some set of methodology.


![Schema](https://github.com/shyamprasath18/Researches/blob/main/Pasted%20image%2020220918161754.png)



### Prevention Mechanisms
- In the previous section, we have highlighted several security risks that the AMI is vulnerable to. 
- And hence, proper security countermeasures must be addressed to maintain the availability and to enhance the efficiency of the AMI.

```
- Encryption
- Authentication Mechanism
- Availability Mechanism
- Jamming-Prevention Mechanism
```


### Conclusion
- However, we’re confident that the **Smart Metering System** strikes the best balance between security and business needs, whilst meeting **broader policy and national security objectives**
- IoT devices are often seen as attractive targets to attackers, because of quantity. So many, billions and billions, are shipped with basic security settings, and these embedded security systems are usually never updated in order to patch against security vulnerabilities.
- So , these should be considered and so many security risks can be prevented.

### References 
[https://www.tripwire.com/state-of-security/security-data-protection/smart-metering-uses-benefits-dangers/](https://www.tripwire.com/state-of-security/security-data-protection/smart-metering-uses-benefits-dangers/)

[https://www.information-age.com/smart-metres-vulnerable-cyber-attacks-123470837/](https://www.information-age.com/smart-metres-vulnerable-cyber-attacks-123470837/)

[https://www.researchgate.net/publication/252050313_Non-invasive_EMI-based_fault_injection_attack_against_cryptographic_modules](https://www.researchgate.net/publication/252050313_Non-invasive_EMI-based_fault_injection_attack_against_cryptographic_modules)

[https://www.sciencedirect.com/science/article/pii/S1877050915008492](https://www.sciencedirect.com/science/article/pii/S1877050915008492)

[https://www.researchgate.net/publication/252050313_Non-invasive_EMI-based_fault_injection_attack_against_cryptographic_modules](https://www.researchgate.net/publication/252050313_Non-invasive_EMI-based_fault_injection_attack_against_cryptographic_modules) - not accessible
