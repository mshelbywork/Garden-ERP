As an end user I should be able to record crop yields in batches. a batch needs a *start date* and a *harvest date*. it also should contain the individual crops planted. 

Individual crops will have an item number, start date estimated harvest date. ideal grow conditions (light, nutrient soultion, PH water temp).

batches should be able to function normally with all crops inputted independently or as a group (i.e 6 lettuce, 4 spinach, 2 arugala). crops lost should be able to be deleted from batches without breaking the batch or the system

User can input pricing details to keep a running ledger of expenses. 


## Crop Production

#### R1. Create planting batches

As a user, I want to record a planting batch with a start date so that I can track crops planted during a particular growing cycle.

#### R2. Record crops in batches

As a user, I want to add one or more crop types to a batch and specify the quantity of each crop so that I can track mixed plantings without entering every plant individually.
    * Do I still want the option to record individually after the fact?

#### R3. Track crop information

As a user, I want to record relevant information for each crop type, including an identifier, estimated harvest date, and ideal growing conditions, so that I can reference this information when growing crops.

#### R4. Record crop losses

As a user, I want to record crop losses and update the remaining crop quantity without invalidating the batch or losing relevant production history.

#### R5. Record harvests

As a user, I want to record the quantity harvested and the harvest date so that I can track the output of each crop batch.

#### Expenses

#### R6. Record expenses

As a user, I want to record purchases and their associated costs so that I can maintain an expense ledger.

#### R7. Review expenses

As a user, I want to review my recorded expenses over time so that I can understand how much I am spending.

**Open Questions**
- Should crops be tracked individually or by quantity?

- Can a batch have multiple harvests?

- Should harvest quantities be recorded by count, weight, or both?
    count for now, weight eventually 

- Should expenses be associated with specific crops or batches?
    I don't think so

- Should the system track harvested produce as inventory?
    Yes

What information does the user need to get out of the system?
    Active inventory, recurring expenses, current batches, total crops planted, total loss by week, yield size by week, view all crop types. Crop specific info