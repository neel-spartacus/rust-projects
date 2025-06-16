fn main() {
    let mut account=BankAccount{
        owner:"Alice".to_string(),
        balance:150.55
    };
    //Immutable borrow to check the balance
    account.check_balance();

    //Mutable borrow to withdraw
    account.withdraw(50.50);

    account.check_balance();
}
struct BankAccount {
    owner:String,
    balance:f64
}

impl BankAccount{
    fn withdraw(&mut self,amount:f64){
        println!("Withdrawing {:.2} from account owned by {}",amount,self.owner);
        self.balance-=amount;

    }

    fn check_balance(&self){
        println!("Account owned by {} have balance : {:.2}",self.owner,self.balance);
    }
}